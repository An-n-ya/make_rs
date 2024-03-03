use std::{cell::RefCell, sync::Arc};

use crate::lexer::Command;
use std::process::Command as SystemCommand;

pub type Node = Arc<RefCell<NodeRaw>>;

pub struct NodeRaw {
    pub node_type: NodeType,
    pub name: String,
    pub dependencies: Vec<Node>,
}

#[derive(PartialEq, Eq)]
pub enum NodeType {
    Target { commands: Vec<Command> },
    File { path: String },
    Unknown,
}

impl NodeRaw {
    pub fn new_target(name: String, commands: Vec<Command>, dep: Vec<Node>) -> Node {
        Arc::new(RefCell::new(Self::new_target_raw(name, commands, dep)))
    }
    pub fn new_target_raw(name: String, commands: Vec<Command>, dep: Vec<Node>) -> NodeRaw {
        Self {
            node_type: NodeType::Target { commands },
            name,
            dependencies: dep,
        }
    }
    pub fn new_unknown(name: String) -> Node {
        Arc::new(RefCell::new(Self {
            node_type: NodeType::Unknown,
            name,
            dependencies: vec![],
        }))
    }
    pub fn new_file(name: String) -> Node {
        Arc::new(RefCell::new(Self {
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
    pub fn set_deps(&mut self, deps: Vec<Node>) {
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
