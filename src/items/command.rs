use crate::scope::Scope;
use crate::transpile::Transpile;
use std::io::Write;
use std::{cell::RefCell, rc::Rc};

use super::expression::{Expression, ExpressionType};
use super::variable::{Type, Variable};
use crate::utils::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandType {
    If,
    While,
    Declare,
    Let,
    Print,
    Input,
}

pub struct Command {
    /// sorted list of subfolder associated to this command
    folders: Vec<String>,

    /// scope that own this command
    scope: Rc<RefCell<Scope>>,

    command_type: CommandType,

    translation_context: Option<TranslationContext>,
}

#[derive(Clone, Default)]
struct TranslationContext {
    token: String,
}

impl Command {
    pub fn get_type(folder_count: usize) -> Option<CommandType> {
        match folder_count {
            0 => Some(CommandType::If),
            1 => Some(CommandType::While),
            2 => Some(CommandType::Declare),
            3 => Some(CommandType::Let),
            4 => Some(CommandType::Print),
            5 => Some(CommandType::Input),
            _ => None,
        }
    }

    fn declare_variable(&mut self) -> std::io::Result<()> {
        let folders_count = subfolder_count(&self.folders[1])?;
        let index = subfolder_count(&self.folders[2])?;

        let mut scope = self.scope.borrow_mut();
        scope.declare_variable(folders_count, index)?;

        if let Some(ctx) = self.translation_context.as_mut() {
            let var = scope.get_variable(index).unwrap();
            ctx.token = format!("let mut var_{index}: {};", var.get_type().as_str());
        }

        Ok(())
    }

    /// 'let' instruction
    fn store_expression(&mut self) -> std::io::Result<()> {
        let var_index = subfolder_count(&self.folders[1])?;

        let mut exp = Expression::new(&self.folders[2], &self.scope)?;
        let value = exp.execute()?;

        if let Some(ctx) = self.translation_context.as_mut() {
            let rvalue = exp.transpile()?;

            let scope = self.scope.borrow();
            if scope.get_variable(var_index).is_none() {
                ctx.token += "let mut ";
            }

            if value.get_type() == Type::String && exp.expression_type == ExpressionType::Variable {
                // make sure to not take ownership of the new string
                ctx.token += &format!("var_{var_index} = {rvalue}.clone();");
            } else {
                ctx.token += &format!("var_{var_index} = {rvalue};");
            }
        }

        let mut scope = self.scope.borrow_mut();
        scope.set_or_create_variable(var_index, value)?;

        Ok(())
    }

    fn print_expression(&self) -> std::io::Result<()> {
        let exp = Expression::new(&self.folders[1], &self.scope)?;
        let value = exp.execute()?;

        print!("{value}");
        std::io::stdout().flush().unwrap();

        Ok(())
    }

    fn execute_in_new_scope(&self, parent_folder: &str) -> std::io::Result<()> {
        let subfolders = sorted_subfolders(parent_folder)?;
        let scope = Rc::new(RefCell::new(Scope::new(Some(self.scope.clone()))));

        for folder in subfolders {
            let mut cmd = Command::new(&folder, &scope)?;
            cmd.run()?;
        }

        Ok(())
    }

    fn get_input(&self) -> std::io::Result<Variable> {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        let var = if let Ok(int) = input.parse::<i32>() {
            Variable::Int(Some(int))
        } else if let Ok(float) = input.parse::<f32>() {
            Variable::Float(Some(float))
        } else if input.len() == 1 {
            Variable::Char(Some(input.chars().next().unwrap()))
        } else {
            Variable::String(Some(input.to_owned()))
        };

        Ok(var)
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        match self.command_type {
            CommandType::Declare => {
                self.declare_variable()?;
            }
            CommandType::Let => {
                self.store_expression()?;
            }
            CommandType::Print => {
                self.print_expression()?;
            }
            CommandType::If => {
                let exp = Expression::new(&self.folders[1], &self.scope)?;
                let value = exp.execute()?;

                if value.is_truthy() {
                    self.execute_in_new_scope(&self.folders[2])?;
                }
            }
            CommandType::While => loop {
                let exp = Expression::new(&self.folders[1], &self.scope)?;
                let value = exp.execute()?;

                if value.is_truthy() {
                    self.execute_in_new_scope(&self.folders[2])?;
                } else {
                    break;
                }
            },
            CommandType::Input => {
                let value = self.get_input()?;
                let var_index = subfolder_count(&self.folders[1])?;

                let mut scope = self.scope.borrow_mut();
                scope.set_or_create_variable(var_index, value)?;
            }
        }

        Ok(())
    }

    pub fn new(folder: &str, scope: &Rc<RefCell<Scope>>) -> std::io::Result<Self> {
        let subfolders = sorted_subfolders(folder)?;
        if subfolders.is_empty() {
            return Err(input_error(format!(
                "{folder}: invalid command, folder is empty"
            )));
        }

        let count = subfolder_count(&subfolders[0])?;
        let command_type = Command::get_type(count);

        if command_type.is_none() {
            let base = std::path::Path::new(folder);
            return Err(input_error(format!(
                "{} : invalid command type, {count} subfolders found",
                base.join(std::path::Path::new(&subfolders[0]))
                    .to_str()
                    .unwrap()
            )));
        }
        let command_type = command_type.unwrap();

        let folder_count = subfolders.len();
        if command_type == CommandType::Input || command_type == CommandType::Print {
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
            scope: scope.clone(),
            translation_context: None,
        };

        Ok(cmd)
    }
}

impl Transpile for Command {
    fn transpile(&mut self) -> std::io::Result<String> {
        self.translation_context = Some(TranslationContext::default());
        let ctx = self.translation_context.as_mut().unwrap();

        match self.command_type {
            CommandType::Declare => {
                self.declare_variable()?;
            }

            CommandType::Let => {
                self.store_expression()?;
            }

            CommandType::If => {
                let mut exp = Expression::new(&self.folders[1], &self.scope)?;
                let token = exp.transpile()?;
                let value = exp.execute()?;

                let token = match exp.expression_type {
                    ExpressionType::EqualTo
                    | ExpressionType::GreaterThan
                    | ExpressionType::LessThan => token,
                    _ => match value.get_type() {
                        Type::Char => format!("{token} != '\0'"),
                        Type::Float => format!("{token} != 0.0"),
                        Type::Int => format!("{token} != 0"),
                        Type::String => {
                            if exp.expression_type == ExpressionType::Add {
                                format!("!({token}).is_empty()")
                            } else {
                                format!("!{token}.is_empty()")
                            }
                        }
                    },
                };

                ctx.token = format!("\nif {token} {{\n");

                let subfolders = sorted_subfolders(&self.folders[2])?;
                let scope = Rc::new(RefCell::new(Scope::new(Some(self.scope.clone()))));

                for folder in subfolders {
                    let mut cmd = Command::new(&folder, &scope)?;

                    for line in cmd.transpile()?.split("\n") {
                        ctx.token += &format!("\t{line}\n");
                    }
                }

                ctx.token += "}";
            }

            CommandType::While => {
                let mut exp = Expression::new(&self.folders[1], &self.scope)?;
                let token = exp.transpile()?;
                let value = exp.execute()?;

                let token = match exp.expression_type {
                    ExpressionType::EqualTo
                    | ExpressionType::GreaterThan
                    | ExpressionType::LessThan => token,
                    _ => match value.get_type() {
                        Type::Char => format!("{token} != '\0'"),
                        Type::Float => format!("{token} != 0.0"),
                        Type::Int => format!("{token} != 0"),
                        Type::String => {
                            if exp.expression_type == ExpressionType::Add {
                                format!("!({token}).is_empty()")
                            } else {
                                format!("!{token}.is_empty()")
                            }
                        }
                    },
                };

                ctx.token = format!("\nwhile {token} {{\n");

                let subfolders = sorted_subfolders(&self.folders[2])?;
                let scope = Rc::new(RefCell::new(Scope::new(Some(self.scope.clone()))));

                for folder in subfolders {
                    let mut cmd = Command::new(&folder, &scope)?;

                    for line in cmd.transpile()?.split("\n") {
                        ctx.token += &format!("\t{line}\n");
                    }
                }

                ctx.token += "}";
            }

            CommandType::Print => {
                let mut exp = Expression::new(&self.folders[1], &self.scope)?;
                let token = exp.transpile()?;

                ctx.token += &format!("print!(\"{{}}\", {token});\n");
                ctx.token += "std::io::stdout().flush().unwrap();\n";
            }

            CommandType::Input => {
                let var_index = subfolder_count(&self.folders[1])?;
                let mut scope = self.scope.borrow_mut();

                if scope.get_variable(var_index).is_none() {
                  ctx.token += &format!("let mut var_{var_index} = String::new();\n");
                }
                ctx.token += &format!("std::io::stdin().readline(&mut var_{var_index}).unwrap();\n");

                scope.set_or_create_variable(var_index, Variable::String(Some(String::new())))?;
            }
        }

        Ok(self.translation_context.clone().unwrap().token)
    }
}
