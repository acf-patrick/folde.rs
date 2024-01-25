use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::items::Variable;

pub struct Scope {
    variables: HashMap<usize, Variable>,
    parent: Option<Rc<RefCell<Scope>>>,
    child: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            parent: None,
            child: None,
        }
    }

    /// Declare a new variable in this scope
    ///
    /// Params :
    /// - folders_count : variable type is indentified with this parameter
    /// - index : index of the variable in this scope. Variable will bear the name 'var_{index}'
    pub fn declare_variable(&mut self, folders_count: usize, index: usize)  {
        if self.variables.get(&index).is_some() {
            panic!("var_{index} declared more than once");
        }

        match Variable::new(folders_count) {
            Ok(variable) => self.variables.insert(index, variable),
            Err(err) => panic!("{err}"),
        };
    }

    pub fn create_int(init: i32) -> Variable {
        Variable::Int(Some(init))
    }

    pub fn create_float(init: f32) -> Variable {
        Variable::Float(Some(init))
    }

    pub fn create_string(init: String) -> Variable {
        Variable::String(Some(init))
    }

    pub fn create_char(init: char) -> Variable {
        Variable::Char(Some(init))
    }
}
