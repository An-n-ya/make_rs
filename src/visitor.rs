use crate::parser::{AssignStmt, DirectiveStmt, RuleStmt, Visitor};

#[derive(Clone, Copy)]
pub struct PrintVisitor {}

#[derive(Clone, Copy)]
pub struct NodeVisitor {}

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

impl Visitor for NodeVisitor {
    fn visit_rule(&mut self, t: &RuleStmt) {
        todo!()
    }

    fn visit_assign(&mut self, t: &AssignStmt) {
        todo!()
    }

    fn visit_directive(&mut self, t: &DirectiveStmt) {
        todo!()
    }
}
