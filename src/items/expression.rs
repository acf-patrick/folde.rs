use crate::{
    items::variable::Type, scope::Scope, transpile::Transpile, utils::{get_byte, input_error, sorted_subfolders, subfolder_count}
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::variable::Variable;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct Expression {
    pub expression_type: ExpressionType,

    /// sorted list of folders associated to this expression
    folders: Vec<String>,

    /// scope that own this expression
    scope: Rc<RefCell<Scope>>,
}

impl Expression {
    pub fn get_type(folder_count: usize) -> Option<ExpressionType> {
        match folder_count {
            0 => Some(ExpressionType::Variable),
            1 => Some(ExpressionType::Add),
            2 => Some(ExpressionType::Substract),
            3 => Some(ExpressionType::Multiply),
            4 => Some(ExpressionType::Divide),
            5 => Some(ExpressionType::LiteralValue),
            6 => Some(ExpressionType::EqualTo),
            7 => Some(ExpressionType::GreaterThan),
            8 => Some(ExpressionType::LessThan),
            _ => None,
        }
    }

    fn get_literal_value(&self) -> std::io::Result<Variable> {
        let var_type = Type::from(subfolder_count(&self.folders[1])?);

        let value_folders = sorted_subfolders(&self.folders[2])?;
        match var_type {
            Type::Int | Type::Float => {
                if value_folders.len() > 4 {
                    return Err(input_error(format!(
                        "{} : invalid literal value, Int and Float are 32-bit but found {} subfolders",
                        self.folders[2],
                        value_folders.len()
                    )));
                }
            }
            Type::Char => {
                if value_folders.len() > 1 {
                    return Err(input_error(format!(
                        "{} : invalid literal value, Char should be one byte unicode but found {} subfolders", 
                        self.folders[2], 
                        value_folders.len()
                    )));
                }
            }
            _ => {}
        }

        let mut value: Vec<u8> = vec![];
        
        for folder in value_folders {
            let byte = get_byte(&folder)?;
            value.push(byte);
        }

        let var = match var_type {
            Type::Int => {
                let mut bytes: [u8; 4] = [0; 4];
                for i in 0..value.len() {
                    bytes[i] = value[i];
                }

                Variable::Int(Some(i32::from_ne_bytes(bytes)))
            }

            Type::Float => {
                let mut bytes: [u8; 4] = [0; 4];
                for i in 0..value.len() {
                    bytes[i] = value[i];
                }

                Variable::Float(Some(f32::from_ne_bytes(bytes)))
            }

            Type::Char => {
                Variable::Char(Some(char::from(value[0])))
            }

            Type::String => {
                let mut str_value = String::new();
                for byte in value {
                    str_value.push(char::from(byte));
                }
                
                Variable::String(Some(str_value))
            }
        };

        Ok(var)
    }

    pub fn execute(&self) -> std::io::Result<Variable> {
        // expression does not have to mutate its scope
        let scope = self.scope.borrow();

        if self.expression_type == ExpressionType::Variable {
            let folder_count = subfolder_count(&self.folders[1])?;

            if let Some(var) = scope.get_variable(folder_count) {
                if var.is_null() {
                    Err(input_error(format!("{} : use of uninitialized variable var_{folder_count}", self.folders[1])))
                } else {
                    Ok(var)
                }
            } else {
                Err(input_error(format!(
                    "{} : expression error, variable var_{folder_count} does not exist",
                    self.folders[1]
                )))
            }
        } else if self.expression_type == ExpressionType::LiteralValue {
            self.get_literal_value()
        } else {
            let first = Expression::new(&self.folders[1], &self.scope)?;
            let second = Expression::new(&self.folders[2], &self.scope)?;

            let a = first.execute()?;
            let b = second.execute()?;

            match self.expression_type {
                ExpressionType::Add => Ok(a + b),

                ExpressionType::Substract => Ok(a - b),

                ExpressionType::Multiply => Ok(a * b),

                ExpressionType::Divide => Ok(a / b),

                ExpressionType::EqualTo => Ok(Variable::Int(Some((a == b) as i32))),

                ExpressionType::GreaterThan => Ok(Variable::Int(Some((a > b) as i32))),

                ExpressionType::LessThan => Ok(Variable::Int(Some((a < b) as i32))),

                _ => Ok(Variable::Int(None)),
            }
        }
    }

    pub fn new(folder: &str, scope: &Rc<RefCell<Scope>>) -> std::io::Result<Self> {
        let subfolders = sorted_subfolders(&folder)?;
        if subfolders.is_empty() {
            return Err(input_error(format!(
                "{folder} : invalid expression, empty folder"
            )));
        }

        let folder_count = subfolder_count(&subfolders[0])?;
        let expression_type = Expression::get_type(folder_count);
        if expression_type.is_none() {
            return Err(input_error(format!(
                "{folder} : invalid expression type, found {folder_count} folders."
            )));
        }

        Ok(Expression {
            expression_type: expression_type.unwrap(),
            folders: subfolders,
            scope: scope.clone(),
        })
    }
}

impl Transpile for Expression {
  fn transpile(&mut self) -> std::io::Result<String> {
      match self.expression_type {
        ExpressionType::Variable => {
          self.execute()?;

          let folder_count = subfolder_count(&self.folders[1])?;
          Ok(format!("var_{folder_count}"))
        }

        ExpressionType::LiteralValue => {
          let value = match self.get_literal_value()? {
            Variable::Char(value) => format!("'{}'", value.unwrap()),
            Variable::Float(value) => format!("{} as f32", value.unwrap()),
            Variable::Int(value) => value.unwrap().to_string(),
            Variable::String(value) => {
              let value = value.unwrap();
              if value.is_empty() {
                "String::new()".to_owned()
              } else {
                format!("\"{}\".to_owned()", value)
              }
            }
          };

          Ok(value)
        }

        _ => {
          let left: String;
          {
            let mut exp = Self::new(&self.folders[1], &self.scope)?;
            if exp.expression_type != ExpressionType::Variable && exp.expression_type != ExpressionType::LiteralValue {
              left = format!("({})", exp.transpile()?);
            } else {
              left = exp.transpile()?;
            }
          }
          
          let right: String;
          let mut right_is_string = false;
          {
            let mut exp = Self::new(&self.folders[2], &self.scope)?;
            if exp.expression_type != ExpressionType::Variable && exp.expression_type != ExpressionType::LiteralValue {
              right = format!("({})", exp.transpile()?);
            } else {
              right = exp.transpile()?;
              
              let value_type = exp.execute()?.get_type();
              right_is_string = value_type == Type::String; 
            }
          }

          let operators = HashMap::from([
            (ExpressionType::Add, "+"),
            (ExpressionType::Substract, "-"),
            (ExpressionType::Multiply, "*"),
            (ExpressionType::Divide, "/"),
            (ExpressionType::GreaterThan, ">"),
            (ExpressionType::LessThan, "<"),
            (ExpressionType::EqualTo, "=="),
          ]);

          if right_is_string && self.expression_type == ExpressionType::Add {
            Ok(format!("{left} + &{right}"))
          } else {
            Ok(format!("{left} {} {right}", operators.get(&self.expression_type).unwrap()))
          }
        }
      }
  }
}