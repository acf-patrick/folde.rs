use crate::{
    scope::Scope,
    utils::{input_error, sorted_subfolders, subfolder_count},
};
use std::{cell::RefCell, rc::Rc};

use super::variable::Variable;

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

    pub fn execute(&self) -> std::io::Result<Variable> {
        let scope = self.scope.borrow();

        match self.expression_type {
            ExpressionType::Variable => {
                let folder_count = subfolder_count(&self.folders[1])?;

                if let Some(var) = scope.get_variable(folder_count) {
                    Ok(var)
                } else {
                    Err(input_error(format!(
                        "{} : expression error, variable var_{folder_count} does not exist",
                        self.folders[1]
                    )))
                }
            }

            ExpressionType::Add => {
                let first = Expression::new(&self.folders[1], &self.scope)?;
                let second = Expression::new(&self.folders[2], &self.scope)?;

                let res1 = first.execute()?;
                let res2 = second.execute()?;
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
