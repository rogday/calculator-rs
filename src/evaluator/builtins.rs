use super::operations::{Args, Associativity, Op, Type};

use enum_map::{enum_map, Enum, EnumMap};
use num_traits::Pow;
use once_cell::sync::Lazy;

type F = fn(Args) -> Type;

#[derive(Enum, Clone, Debug, Copy)]
pub enum InfixOperators {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    UnaryMinus,
}

pub struct Builtin {
    pub op: Op,
    pub f:  F,
}

macro_rules! make_builtin {
    ($($name:ident |$($names:ident),+| $body:block),+) => {
        $(
            #[allow(non_snake_case)]
            fn $name<'a>(args: Args<'a>) -> Type {
                match args {
                    [ $($names),+] => $body,
                    _ => unreachable!(),
                }
            }
        )+
    }
}

macro_rules! make_mapping {
    ($($variant:ident, $op:expr);+) => {
        enum_map!{
            $(InfixOperators::$variant => Builtin { f: $variant, op: $op }),+
        }
    };
}

#[rustfmt::skip]
make_builtin![
    Pow | a, b | { Pow::pow(a, b) },
    Add | a, b | { a + b },
    Sub | a, b | { a - b },
    Mul | a, b | { a * b },
    Div | a, b | { a / b },
    UnaryMinus | a | { -a }
];

pub static BUILTINS: Lazy<EnumMap<InfixOperators, Builtin>> = Lazy::new(|| {
    use Associativity::*;

    make_mapping![
        Add,        Op { arity: 2, prec: 2, assoc: Left  };
        Sub,        Op { arity: 2, prec: 2, assoc: Left  };
        Mul,        Op { arity: 2, prec: 3, assoc: Left  };
        Div,        Op { arity: 2, prec: 3, assoc: Left  };
        Pow,        Op { arity: 2, prec: 4, assoc: Right };
        UnaryMinus, Op { arity: 1, prec: 5, assoc: Right }
    ]
});
