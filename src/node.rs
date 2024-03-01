use std::{cell::RefCell, rc::Rc};

use crate::lexer::Command;
use std::process::Command as SystemCommand;

pub struct Node {
    pub node_type: NodeType,
    pub name: String,
    pub dependencies: Vec<Rc<RefCell<Node>>>,
}

#[derive(PartialEq, Eq)]
pub enum NodeType {
    Target { commands: Vec<Command> },
    File { path: String },
    Unknown,
}

impl Node {
    pub fn new_target(
        name: String,
        commands: Vec<Command>,
        dep: Vec<Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self::new_target_raw(name, commands, dep)))
    }
    pub fn new_target_raw(
        name: String,
        commands: Vec<Command>,
        dep: Vec<Rc<RefCell<Node>>>,
    ) -> Node {
        Self {
            node_type: NodeType::Target { commands },
            name,
            dependencies: dep,
        }
    }
    pub fn new_unknown(name: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            node_type: NodeType::Unknown,
            name,
            dependencies: vec![],
        }))
    }
    pub fn new_file(name: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            node_type: NodeType::File {
                path: ".".to_string(),
            },
            name,
            dependencies: vec![],
        }))
    }

    pub fn execute(&self) {
        println!("executing target {}", self.name);
        match &self.node_type {
            NodeType::Target { commands } => {
                for command in commands {
                    println!("{command:?}");
                    command.execute();
                }
            }
            NodeType::File { .. } => { /* do nothing */ }
            NodeType::Unknown => unreachable!(),
        };
    }

    pub fn set_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
    pub fn set_deps(&mut self, deps: Vec<Rc<RefCell<Node>>>) {
        self.dependencies = deps;
    }
}

impl Command {
    pub fn execute(&self) {
        SystemCommand::new(&self.command)
            .args(&self.args)
            .output()
            .expect(&format!("execute command {} failed", &self.command));
    }
}
