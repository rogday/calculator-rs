use super::builtins::InfixOperators;
use super::control::Control;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Func<'fname> {
    FunStart(&'fname str),
    Comma,
    FunEnd,
}

#[derive(Clone, Debug)]
pub enum OperationType<'fname> {
    Control(Control),
    BuiltinFun(InfixOperators),
    Func(Func<'fname>),
}

#[derive(Debug, Copy, Clone)]
pub struct Op {
    pub arity: usize,
    pub prec:  u8,
    pub assoc: Associativity,
}

pub type Type = f64;
pub type Args<'a> = &'a [Type];

pub type FunType = Box<dyn Fn(Args) -> Type + Send + Sync>;
