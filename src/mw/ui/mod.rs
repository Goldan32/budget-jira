use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
pub enum InputCommand {
    Add(Task),
    Ls(Status),
}

#[allow(unused)]
pub trait FrontEndCapabilities {
    fn add(name: String, description: String, duedate: String) -> Task;
    fn ls(status: String) -> Status;
}

pub trait FrontEndInput {
    fn new() -> Self;
    fn execute(&self) -> InputCommand;
}
