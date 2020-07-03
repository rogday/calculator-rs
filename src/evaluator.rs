pub mod builtins;
pub mod control;
pub mod operations;

use operations::{Associativity, FunType, Func, Op, OperationType};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Token<'a> {
    Number(f64),
    Operation(OperationType<'a>),
}

#[derive(Clone, Copy, Debug)]
pub enum EvalError {
    NotEnoughArguments,
    LogicError,
}

struct UserFunction {
    arity: usize,
    f:     FunType,
}

pub struct Evaluator {
    functions: HashMap<String, UserFunction>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { functions: Default::default() }
    }

    fn operator_lookup(&self, op: &OperationType) -> Op {
        match op {
            OperationType::Control(ctrl) => control::CONTROL[*ctrl],
            OperationType::BuiltinFun(math) => builtins::BUILTINS[*math].op,

            OperationType::Func(Func::FunStart(name)) => {
                Op { arity: self.functions[*name].arity, prec: 1, assoc: Associativity::Left }
            }
            OperationType::Func(Func::FunEnd) => {
                Op { arity: 0, prec: 7, assoc: Associativity::Left }
            }

            OperationType::Func(Func::Comma) => {
                Op { arity: 0, prec: 7, assoc: Associativity::Left }
            }
        }
    }

    pub fn add_fn(&mut self, name: String, arity: usize, f: FunType) {
        self.functions.insert(name, UserFunction { arity, f });
    }

    pub fn eval(&self, bytecode: &[Token]) -> Result<f64, EvalError> {
        let mut numbers = vec![];
        let mut operators = vec![];
        let mut args = Vec::with_capacity(2);

        for token in bytecode {
            // println!("Token: {:?}\nnumbers: {:?}\noperators: {:?}\n", token, numbers, operators);
            match token {
                Token::Number(number) => numbers.push(*number),
                Token::Operation(op) => {
                    while let Some(prev_op) = operators.last() {
                        let op_info = self.operator_lookup(op);
                        let prev_op_info = self.operator_lookup(prev_op);

                        //TODO: ) and ( should cancel each other out, same is true for FuncStart and FuncEnd
                        // NOTE: >= is right assoc, then > should be left
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
                            OperationType::BuiltinFun(name) => {
                                numbers.push((builtins::BUILTINS[*name].f)(&args))
                            }
                            OperationType::Func(Func::FunStart(name)) => {
                                numbers.push((self.functions[*name].f)(&args))
                            }
                            _ => (),
                        }

                        operators.pop();
                    }
                    operators.push(op.clone());
                }
            }
        }

        if let [ret] = numbers[..] {
            Ok(ret)
        } else {
            Err(EvalError::LogicError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        builtins::InfixOperators, control::Control, operations::OperationType, Evaluator, Token,
    };

    const EPS: f64 = 0.0001;

    fn assert_approx(a: f64, b: f64) {
        assert!((a - b).abs() < EPS)
    }

    #[test]
    fn precedence_test() {
        use Control::*;
        use Token::*;

        let eval = Evaluator::new();

        let tokens = vec![
            Operation(OperationType::Control(OpenBracket)),
            Number(43.),
            Operation(OperationType::Control(CloseBracket)),
            Operation(OperationType::BuiltinFun(InfixOperators::Add)),
            Operation(OperationType::BuiltinFun(InfixOperators::UnaryMinus)),
            Number(2.),
            Operation(OperationType::BuiltinFun(InfixOperators::Div)),
            Number(0.5),
            Operation(OperationType::BuiltinFun(InfixOperators::Mul)),
            Number(2.),
            Operation(OperationType::BuiltinFun(InfixOperators::Sub)),
            Number(0.0),
            Operation(OperationType::BuiltinFun(InfixOperators::Pow)),
            Number(0.0),
            Operation(OperationType::Control(EndExpr)),
        ];

        assert_approx(eval.eval(&tokens).unwrap(), (43.) + -2. / 0.5 * 2. - 0f64.powf(0.));
    }
}
