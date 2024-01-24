#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionType {
    Variable,
    Add,
    Substract,
    Multiply,
    Divide,
    LiteralValue,
    EqualTo,
    GreaterThan,
    LessThan,
}
