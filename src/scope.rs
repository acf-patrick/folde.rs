use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    items::variable::{Type as VariableType, Variable},
    utils::input_error,
};

pub struct Scope {
    variables: HashMap<usize, Variable>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Scope {
            variables: HashMap::new(),
            parent,
        }
    }

    /// Declare a new variable in this scope
    ///
    /// Params :
    /// - folders_count : variable type is indentified with this parameter
    /// - index : index of the variable in this scope. Variable will bear the name 'var_{index}'
    pub fn declare_variable(&mut self, folders_count: usize, index: usize) -> std::io::Result<()> {
        if self.variables.get(&index).is_some() {
            return Err(input_error(format!("var_{index} declared more than once")));
        }

        let variable = Variable::new(folders_count)?;
        self.variables.insert(index, variable);

        Ok(())
    }

    pub fn declare_variable_with_type(
        &mut self,
        var_type: VariableType,
        index: usize,
    ) -> std::io::Result<()> {
        if self.variables.get(&index).is_some() {
            return Err(input_error(format!("var_{index} declared more than once")));
        }

        let var = Variable::from(var_type);
        self.variables.insert(index, var);

        Ok(())
    }

    pub fn get_variable(&self, index: usize) -> Option<Variable> {
        if let Some(var) = self.variables.get(&index) {
            return Some(var.clone());
        }

        if let Some(scope) = &self.parent {
            let parent = scope.borrow_mut();
            if let Some(var) = parent.get_variable(index) {
                return Some(var);
            }
        }

        None
    }

    fn _set_variable(&mut self, index: usize, value: Variable) -> std::io::Result<bool> {
        if self.variables.contains_key(&index) {
            {
                let var = self.variables.get(&index).unwrap();
                if !var.same_as(&value) {
                    return Err(input_error(format!(
                        "Cannot assign value of type {:?} to variable of type {:?}",
                        value.get_type(),
                        var.get_type()
                    )));
                }
            }

            self.variables.insert(index, value);

            return Ok(true);
        }

        if let Some(scope) = &self.parent {
            let mut parent = scope.borrow_mut();
            if parent._set_variable(index, value)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn set_variable(&mut self, index: usize, value: Variable) -> std::io::Result<()> {
        if self._set_variable(index, value)? {
            Ok(())
        } else {
            Err(input_error(format!(
                "Use of undeclared variable var_{index}"
            )))
        }
    }

    pub fn set_or_create_variable(&mut self, index: usize, value: Variable) -> std::io::Result<()> {
        if !self._set_variable(index, value.clone())? {
            let var_type = value.get_type();
            self.declare_variable_with_type(var_type, index).ok();
            self.set_variable(index, value).ok();
        }

        Ok(())
    }
}
