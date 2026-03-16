use crate::value::Value;

pub enum Action {
    Return(Value),
    None,
    Break,
    Continue,
}
