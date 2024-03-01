use std::{cell::RefCell, collections::HashMap, env, fs, ops::IndexMut, rc::Rc};

use crate::{
    node::{Node, NodeType},
    parser::{walk_stmt, AssignStmt, DirectiveStmt, RuleStmt, Visitor},
};

pub struct PrintVisitor {}

pub struct NodeVisitor {
    node_map: HashMap<String, Rc<RefCell<Node>>>,
    default_target: Option<Rc<RefCell<Node>>>,
}

impl PrintVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for PrintVisitor {
    fn visit_rule(&mut self, t: &RuleStmt) {
        println!(
            "RuleStmt{{target:{}, prerequisite: {:?}, command: {:?}}}",
            t.target,
            t.prerequisite,
            t.commands
                .iter()
                .fold("".to_string(), |acc, command| acc + &command.command + " ")
        )
    }

    fn visit_assign(&mut self, t: &AssignStmt) {
        todo!()
    }

    fn visit_directive(&mut self, t: &DirectiveStmt) {
        todo!()
    }
}

impl NodeVisitor {
    pub fn new() -> Self {
        Self {
            node_map: HashMap::default(),
            default_target: None,
        }
    }

    pub fn run(&self, target: Option<String>) {
        let target_node = target.map_or(
            self.default_target
                .as_ref()
                .expect("cannot find any target")
                .clone(),
            |name| {
                self.node_map
                    .get(&name)
                    .expect(&format!("cannot find target {}", name))
                    .clone()
            },
        );

        let tasks = Self::topo_sort(target_node);

        for task in tasks {
            task.borrow().execute();
        }
    }

    fn topo_sort(node: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
        #[derive(PartialEq, Eq)]
        enum Status {
            Visiting,
            Visited,
        }
        let mut status_map: HashMap<String, Status> = HashMap::new();
        let mut res: Vec<Rc<RefCell<Node>>> = vec![];

        fn dfs(
            node: Rc<RefCell<Node>>,
            status_map: &mut HashMap<String, Status>,
            res: &mut Vec<Rc<RefCell<Node>>>,
        ) -> bool {
            let node_ref = node.borrow();
            let name = node_ref.name.clone();
            let dependencies = &node_ref.dependencies;
            status_map.insert(name.clone(), Status::Visiting);
            for nei in dependencies {
                let nei_name = nei.borrow().name.clone();
                if status_map.contains_key(&nei_name) && status_map[&nei_name] == Status::Visiting {
                    return false;
                }
                let nei = nei.clone();
                if !status_map.contains_key(&nei_name) && !dfs(nei, status_map, res) {
                    return false;
                }
            }
            res.push(node.clone());
            status_map.insert(name, Status::Visited);
            true
        }
        dfs(node, &mut status_map, &mut res);

        res
    }
}

impl Visitor for NodeVisitor {
    fn visit_rule(&mut self, t: &RuleStmt) {
        let name = &t.target;
        let commands = t.commands.clone();
        let mut deps = vec![];
        for dep in &t.prerequisite {
            if self.node_map.contains_key(dep) {
                deps.push(self.node_map[dep].clone());
            } else {
                let new_node = Node::new_unknown(dep.clone());
                deps.push(new_node.clone());
                self.node_map.insert(dep.clone(), new_node);
            }
        }
        if self.node_map.contains_key(name) {
            if let Some(node_inside) = self.node_map.get_mut(name) {
                if &node_inside.borrow().node_type == &NodeType::Unknown {
                    *node_inside.borrow_mut() = Node::new_target_raw(name.clone(), commands, deps);
                } else if let NodeType::File { .. } = &node_inside.borrow().node_type {
                    // overwite the existing file
                    // *node_inside.borrow_mut() = Node::new_target_raw(name.clone(), commands, deps);
                } else {
                    panic!("replicated target {}", name);
                }
            }
        } else {
            self.node_map
                .insert(name.clone(), Node::new_target(name.clone(), commands, deps));
        }

        if self.default_target.is_none() {
            self.default_target = Some(self.node_map[name].clone());
        }
    }

    fn visit_assign(&mut self, t: &AssignStmt) {
        todo!()
    }

    fn visit_directive(&mut self, t: &DirectiveStmt) {
        todo!()
    }

    fn visit_program(&mut self, t: &crate::parser::Program) {
        // find all the local files
        let current_dir = env::current_dir().expect("failed to load current directory");
        for entry in fs::read_dir(current_dir).expect("failed to open current dir") {
            let entry = entry.expect("unable to open entry");
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();
                self.node_map.insert(name.clone(), Node::new_file(name));
            }
        }
        for stmt in &t.statements {
            walk_stmt(self, stmt);
        }
    }
}
