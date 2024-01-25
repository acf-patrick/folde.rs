pub mod command;
pub mod expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Int(Option<i32>),
    Float(Option<f32>),
    String(Option<std::string::String>),
    Char(Option<char>),
}

impl Variable {
    pub fn new(folders_count: usize) -> std::io::Result<Self> {
        match folders_count {
            0 => Ok(Variable::Int(None)),
            1 => Ok(Variable::Float(None)),
            2 => Ok(Variable::String(None)),
            3 => Ok(Variable::Char(None)),
            count => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid variable type : {count} folders found."),
            )),
        }
    }
}
