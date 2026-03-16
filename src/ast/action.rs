use crate::ast::value::Value;

pub enum Action {
    Return(Value),
    None,
    Break,
    Continue,
}
