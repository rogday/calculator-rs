#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

#[derive(Clone, Copy, enum_map::Enum, Debug)]
enum Control {
    EndExpr,
    Join,
    OpenBracket,
    CloseBracket,
}

#[derive(Clone, Copy, enum_map::Enum, Debug)]
enum Math {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    UnaryMinus,
}

#[derive(Clone, Copy, Debug)]
enum OperationType {
    Control(Control),
    Math(Math),
}

struct Op {
    arity:      usize,
    precedence: u8,
    assoc:      Associativity,
}

#[derive(Clone, Copy, Debug)]
enum Token {
    //Symbol(usize),
    Number(f64),
    Operation(OperationType),
}

#[derive(Clone, Copy, Debug)]
enum EvalError {
    NotEnoughArguments,
    IllegalJoin,
    LogicError,
}

lazy_static::lazy_static! {
    static ref CONTROL: enum_map::EnumMap<Control, Op> = enum_map::enum_map! {
        Control::Join         => Op { arity: 2, precedence: 6, assoc: Associativity::Left },
        Control::EndExpr      => Op { arity: 0, precedence: 0, assoc: Associativity::Left },
        Control::CloseBracket => Op { arity: 0, precedence: 1, assoc: Associativity::Left },
        Control::OpenBracket  => Op { arity: 0, precedence: 7, assoc: Associativity::Left },
    };
}

lazy_static::lazy_static! {
    static ref MATH: enum_map::EnumMap<Math, Op> = enum_map::enum_map! {
        Math::Add        => Op { arity: 2, precedence: 2, assoc: Associativity::Left },
        Math::Sub        => Op { arity: 2, precedence: 2, assoc: Associativity::Left },
        Math::Mul        => Op { arity: 2, precedence: 3, assoc: Associativity::Left },
        Math::Div        => Op { arity: 2, precedence: 3, assoc: Associativity::Left },
        Math::Pow        => Op { arity: 2, precedence: 4, assoc: Associativity::Right },
        Math::UnaryMinus => Op { arity: 1, precedence: 5, assoc: Associativity::Right },

    };
}

fn operator_lookup(op: &OperationType) -> &Op {
    match op {
        OperationType::Control(ctrl) => &CONTROL[*ctrl],
        OperationType::Math(math) => &MATH[*math],
    }
}

fn eval(bytecode: &[Token]) -> Result<f64, EvalError> {
    let mut numbers = vec![];
    let mut operators = vec![];
    let mut args = Vec::with_capacity(2);

    for token in bytecode {
        // println!("Token: {:?}\nnumbers: {:?}\noperators: {:?}\n", token, numbers, operators);
        match token {
            Token::Number(number) => numbers.push(*number),
            Token::Operation(op) => {
                while let Some(prev_op) = operators.last() {
                    // NOTE: >= is right assoc, then > should be left
                    let op_info = operator_lookup(op);
                    let prev_op_info = operator_lookup(prev_op);

                    if op_info.precedence > prev_op_info.precedence
                        || prev_op_info.assoc == Associativity::Right
                            && op_info.precedence == prev_op_info.precedence
                    {
                        break;
                    }

                    let arity = prev_op_info.arity;

                    if numbers.len() < arity {
                        return Err(EvalError::NotEnoughArguments);
                    }

                    args.clear();
                    numbers.drain(numbers.len() - arity..).for_each(|arg| args.push(arg));

                    match prev_op {
                        OperationType::Control(Control::Join) => {
                            if args[0].min(args[1]) < 0. {
                                return Err(EvalError::IllegalJoin);
                            }
                            numbers.push(args[0] * 10. + args[1])
                        }
                        OperationType::Math(m) => numbers.push(match m {
                            Math::Add => args[0] + args[1],
                            Math::Sub => args[0] - args[1],
                            Math::Mul => args[0] * args[1],
                            Math::Div => args[0] / args[1],
                            Math::Pow => args[0].powf(args[1]),
                            Math::UnaryMinus => -args[0],
                        }),
                        OperationType::Control(_) => (),
                    }

                    operators.pop();
                }
                operators.push(*op);
            }
        }
    }

    if let [ret] = numbers[..] {
        Ok(ret)
    } else {
        Err(EvalError::LogicError)
    }
}

#[cfg(test)]
mod tests {
    const EPS: f64 = 0.0001;

    fn assert_approx(a: f64, b: f64) {
        assert!((a - b).abs() < EPS)
    }

    #[test]
    fn precedence_test() {
        use super::{Control::*, Math::*, Token::*, *};

        let tokens = vec![
            Operation(OperationType::Control(OpenBracket)),
            Number(4.),
            Operation(OperationType::Control(Join)),
            Number(3.),
            Operation(OperationType::Control(CloseBracket)),
            Operation(OperationType::Math(Add)),
            Operation(OperationType::Math(UnaryMinus)),
            Number(2.),
            Operation(OperationType::Math(Div)),
            Number(0.5),
            Operation(OperationType::Math(Mul)),
            Number(2.),
            Operation(OperationType::Math(Sub)),
            Number(0.0),
            Operation(OperationType::Math(Pow)),
            Number(0.0),
            Operation(OperationType::Control(EndExpr)),
        ];

        assert_approx(super::eval(&tokens).unwrap(), (43.) + -2. / 0.5 * 2. - 0f64.powf(0.));
    }
}

fn main() {
    use Control::*;
    use Math::*;
    use Token::*;

    //let string = std::env::args().skip(1).take(1).next().unwrap();
    let tokens = vec![
        Number(7.),
        Operation(OperationType::Math(Mul)),
        Number(6.),
        Operation(OperationType::Control(EndExpr)),
    ];

    println!("{:?} = {:?}", tokens, eval(&tokens));
}
