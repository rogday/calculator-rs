use super::{Associativity, Op};
use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Enum, Debug)]
pub enum Control {
    EndExpr,
    OpenBracket,
    CloseBracket,
}

pub static CONTROL: Lazy<EnumMap<Control, Op>> = Lazy::new(|| {
    use Associativity::*;

    enum_map! {
        Control::EndExpr      => Op { arity: 0, prec: 0, assoc: Left },
        Control::CloseBracket => Op { arity: 0, prec: 7, assoc: Left },
        Control::OpenBracket  => Op { arity: 0, prec: 1, assoc: Left },
    }
});
