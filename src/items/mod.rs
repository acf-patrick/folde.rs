pub mod expression;
pub mod command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
}
