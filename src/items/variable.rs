#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Int(Option<i32>),
    Float(Option<f32>),
    String(Option<std::string::String>),
    Char(Option<char>),
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
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

    pub fn get_type(&self) -> Type {
        match self {
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::String(_) => Type::String,
            Variable::Char(_) => Type::Char,
        }
    }

    pub fn same_as(&self, var: &Variable) -> bool {
        self.get_type() == var.get_type()
    }
}

impl From<Type> for Variable {
    fn from(value: Type) -> Self {
        match value {
            Type::Char => Variable::Char(None),
            Type::Int => Variable::Int(None),
            Type::Float => Variable::Float(None),
            Type::String => Variable::String(None),
        }
    }
}