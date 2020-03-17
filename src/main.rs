#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

use enum_map::Enum;

#[derive(Clone, Copy, Enum, Debug)]
enum Control {
    EndExpr,
    Join,
    OpenBracket,
    CloseBracket,
}

#[derive(Clone, Copy, Enum, Debug)]
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
    arity: usize,
    prec:  u8,
    assoc: Associativity,
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

use enum_map::{enum_map, EnumMap};
use once_cell::sync::Lazy;

static CONTROL: Lazy<EnumMap<Control, Op>> = Lazy::new(|| {
    use Associativity::*;

    enum_map! {
        Control::Join         => Op { arity: 2, prec: 6, assoc: Left },
        Control::EndExpr      => Op { arity: 0, prec: 0, assoc: Left },
        Control::CloseBracket => Op { arity: 0, prec: 1, assoc: Left },
        Control::OpenBracket  => Op { arity: 0, prec: 7, assoc: Left },
    }
});

static MATH: Lazy<EnumMap<Math, Op>> = Lazy::new(|| {
    use Associativity::*;

    enum_map! {
        Math::Add        => Op { arity: 2, prec: 2, assoc: Left },
        Math::Sub        => Op { arity: 2, prec: 2, assoc: Left },
        Math::Mul        => Op { arity: 2, prec: 3, assoc: Left },
        Math::Div        => Op { arity: 2, prec: 3, assoc: Left },
        Math::Pow        => Op { arity: 2, prec: 4, assoc: Right },
        Math::UnaryMinus => Op { arity: 1, prec: 5, assoc: Right },

    }
});

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

                    if op_info.prec > prev_op_info.prec
                        || prev_op_info.assoc == Associativity::Right
                            && op_info.prec == prev_op_info.prec
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
