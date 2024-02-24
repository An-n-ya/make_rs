use crate::{
    node::Node,
    parser::{AssignStmt, DirectiveStmt, RuleStmt, VisitorTrait},
};

#[derive(Clone, Copy)]
pub enum Visitor {
    String(PrintVisitor),
    Node(NodeVisitor),
}

impl Visitor {
    pub fn new_string_visitor() -> Self {
        Self::String(PrintVisitor {})
    }
    pub fn new_node_visitor() -> Self {
        Self::Node(NodeVisitor {})
    }
}

#[derive(Clone, Copy)]
pub struct PrintVisitor {}

#[derive(Clone, Copy)]
pub struct NodeVisitor {}

impl VisitorTrait<String> for PrintVisitor {
    fn visit_rule(&self, t: &RuleStmt) -> String {
        format!(
            "RuleStmt{{target:{}, prerequisite: {:?}, command: {:?}}}",
            t.target,
            t.prerequisite,
            t.commands
                .iter()
                .fold("".to_string(), |acc, command| acc + &command.command + " ")
        )
    }

    fn visit_assign(&self, t: &AssignStmt) -> String {
        todo!()
    }

    fn visit_directive(&self, t: &DirectiveStmt) -> String {
        todo!()
    }
}

impl VisitorTrait<Node> for NodeVisitor {
    fn visit_rule(&self, t: &RuleStmt) -> Node {
        todo!()
    }

    fn visit_assign(&self, t: &AssignStmt) -> Node {
        todo!()
    }

    fn visit_directive(&self, t: &DirectiveStmt) -> Node {
        todo!()
    }
}
