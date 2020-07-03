use super::operations::{Associativity, Op};

use enum_map::{enum_map, Enum, EnumMap};
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Enum, Debug)]
pub enum Control {
    EndExpr,
    OpenBracket,
    CloseBracket,
}

pub const CLOSE_BRACKET_PREC: u8 = 1;
pub const OPEN_BRACKET_PREC: u8 = 1;

pub static CONTROL: Lazy<EnumMap<Control, Op>> = Lazy::new(|| {
    use Associativity::*;

    enum_map! {
        Control::EndExpr      => Op { arity: 0, prec: 0,                  assoc: Left },
        Control::CloseBracket => Op { arity: 0, prec: CLOSE_BRACKET_PREC, assoc: Left },
        Control::OpenBracket  => Op { arity: 0, prec: OPEN_BRACKET_PREC,  assoc: Left },
    }
});
