use std::ops::{Add, Div, Mul, Sub};

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
    /// Create a new variable
    ///
    /// Parameter
    /// - folders_count : used to indentify variable type
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

impl Div<Variable> for Variable {
    type Output = Variable;

    fn div(self, other: Variable) -> Self::Output {
        match self {
            Self::Char(_) => {
                panic!("Can not divide a Char");
            }

            Self::String(value) => {
                panic!("Can not divide a String");
            }

            Self::Float(value) => {
                if value.is_none() {
                    panic!("Use of uninitialized float variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not divide Float by Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value / value));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value as f32 / value));
                    }
                }
            }

            Self::Int(value) => {
                if value.is_none() {
                    panic!("Use of ininitialized int variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not divide Int by Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value / value as f32));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value as f32 / value as f32));
                    }
                }
            }
        }
    }
}

impl Mul<Variable> for Variable {
    type Output = Variable;

    fn mul(self, other: Variable) -> Self::Output {
        match self {
            Self::Char(_) => {
                panic!("Can not multiply a Char");
            }

            Self::String(value) => {
                panic!("Can not multiply a String");
            }

            Self::Float(value) => {
                if value.is_none() {
                    panic!("Use of uninitialized float variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not multiply Float by Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value * value));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value as f32 * value));
                    }
                }
            }

            Self::Int(value) => {
                if value.is_none() {
                    panic!("Use of ininitialized int variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not multiply Int by Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value * value as f32));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Int(Some(other_value * value));
                    }
                }
            }
        }
    }
}

impl Sub<Variable> for Variable {
    type Output = Variable;

    fn sub(self, other: Variable) -> Variable {
        match self {
            Self::Char(_) => {
                panic!("Can not substract from a Char");
            }

            Self::String(value) => {
                panic!("Can not substract from a String");
            }

            Self::Float(value) => {
                if value.is_none() {
                    panic!("Use of uninitialized float variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not substract Float to Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value - value));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value as f32 - value));
                    }
                }
            }

            Self::Int(value) => {
                if value.is_none() {
                    panic!("Use of ininitialized int variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not substract Int to Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value - value as f32));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Int(Some(other_value - value));
                    }
                }
            }
        }
    }
}

impl Add<Variable> for Variable {
    type Output = Variable;

    fn add(self, other: Variable) -> Variable {
        match self {
            Self::Char(_) => {
                panic!("Can not add to a Char");
            }

            Self::String(value) => {
                if value.is_none() {
                    panic!("Use of uninitialized float variable");
                }
                let value = value.unwrap();

                if let Self::String(other_value) = other {
                    if other_value.is_none() {
                        panic!("Use of uninitialized string variable");
                    }
                    let other_value = other_value.unwrap();

                    return Variable::String(Some(format!("{value}{other_value}")));
                } else {
                    let other_type = other.get_type();
                    panic!("Can not add String to {:?}", other_type);
                }
            }

            Self::Float(value) => {
                if value.is_none() {
                    panic!("Use of uninitialized float variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not add Float to Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value + value));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value as f32 + value));
                    }
                }
            }

            Self::Int(value) => {
                if value.is_none() {
                    panic!("Use of ininitialized int variable");
                }
                let value = value.unwrap();

                match other {
                    Self::Char(_) | Self::String(_) => {
                        panic!("Can not add Int to Char or String");
                    }
                    Self::Float(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized float variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Float(Some(other_value + value as f32));
                    }
                    Self::Int(other_value) => {
                        if other_value.is_none() {
                            panic!("Use of uninitialized int variable");
                        }
                        let other_value = other_value.unwrap();

                        return Variable::Int(Some(other_value + value));
                    }
                }
            }
        }
    }
}
