use std::{cell::RefCell, rc::Rc};

use crate::utils::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    If(Vec<usize>),
    While(Vec<usize>),
    Declare,
    Let,
    Print,
    Input,
}

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub folders: Vec<String>,
    pub variables: Option<Rc<RefCell<Vec<usize>>>>,
}

fn input_error(msg: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg)
}

impl Command {
    pub fn get_type(folder_count: usize) -> Option<CommandType> {
        match folder_count {
            0 => Some(CommandType::If(vec![])),
            1 => Some(CommandType::While(vec![])),
            2 => Some(CommandType::Declare),
            3 => Some(CommandType::Let),
            4 => Some(CommandType::Print),
            5 => Some(CommandType::Input),
            _ => None,
        }
    }

    pub fn new(folder: &str) -> std::io::Result<Self> {
        let subfolders = sorted_subfolders(folder)?;
        if subfolders.is_empty() {
            return Err(input_error(format!("{folder}: folder is empty")));
        }

        let count = subfolder_count(&subfolders[0])?;
        let command_type = Command::get_type(count);

        if command_type.is_none() {
            let base = std::path::Path::new(folder);
            return Err(input_error(format!(
                "{} : invalid number of subfolder {count}",
                base.join(std::path::Path::new(&subfolders[0]))
                    .to_str()
                    .unwrap()
            )));
        }
        let command_type = command_type.unwrap();

        let folder_count = subfolders.len();
        if command_type != CommandType::Input && command_type != CommandType::Print {
            if folder_count != 2 {
                return Err(input_error(format!(
                    "{folder} : expected 2 folders, {folder_count} found"
                )));
            }
        } else {
            if folder_count != 3 {
                return Err(input_error(format!(
                    "{folder} : expected 3 folders, {folder_count} found"
                )));
            }
        }

        let cmd = Command {
            command_type,
            folders: subfolders,
            variables: None,
        };

        Ok(cmd)
    }
}
