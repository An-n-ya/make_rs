use core::panic;
use std::collections::HashMap;

use crate::{
    lexer::{Command, Lexer, Token},
    node::Node,
    visitor::Visitor,
};

pub type Statement = Box<dyn StatementTrait>;

pub trait StatementTrait {
    fn walk_to_string(&self, visitor: Visitor) -> String;
    fn walk_to_node(&self, visitor: Visitor) -> Node;
}
pub trait VisitorTrait<T> {
    fn visit_rule(&self, t: &RuleStmt) -> T;
    fn visit_assign(&self, t: &AssignStmt) -> T;
    fn visit_directive(&self, t: &DirectiveStmt) -> T;
}

#[derive(Debug)]
pub struct RuleStmt {
    pub target: String,
    pub prerequisite: Vec<String>,
    pub commands: Vec<Command>,
}
pub struct AssignStmt {}
pub struct DirectiveStmt {}

impl StatementTrait for RuleStmt {
    fn walk_to_string(&self, visitor: Visitor) -> String {
        match visitor {
            Visitor::String(v) => v.visit_rule(self),
            Visitor::Node(v) => panic!("wrong visitor"),
        }
    }
    fn walk_to_node(&self, visitor: Visitor) -> Node {
        match visitor {
            Visitor::Node(v) => v.visit_rule(self),
            Visitor::String(v) => panic!("wrong visitor"),
        }
    }
}
impl StatementTrait for AssignStmt {
    fn walk_to_string(&self, visitor: Visitor) -> String {
        match visitor {
            Visitor::String(v) => v.visit_assign(self),
            Visitor::Node(v) => panic!("wrong visitor"),
        }
    }
    fn walk_to_node(&self, visitor: Visitor) -> Node {
        match visitor {
            Visitor::Node(v) => v.visit_assign(self),
            Visitor::String(v) => panic!("wrong visitor"),
        }
    }
}
impl StatementTrait for DirectiveStmt {
    fn walk_to_string(&self, visitor: Visitor) -> String {
        match visitor {
            Visitor::String(v) => v.visit_directive(self),
            Visitor::Node(v) => panic!("wrong visitor"),
        }
    }
    fn walk_to_node(&self, visitor: Visitor) -> Node {
        match visitor {
            Visitor::Node(v) => v.visit_directive(self),
            Visitor::String(v) => panic!("wrong visitor"),
        }
    }
}

pub struct Program {
    statements: Vec<Statement>,
    symbol_table: HashMap<Token, String>,
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
}

impl Parser {
    pub fn new(contents: String) -> Self {
        Self {
            lexer: Lexer::new(contents),
            cur_token: None,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.cur_token = self.lexer.next();
        self.cur_token.clone()
    }

    fn cur_token(&self) -> Option<Token> {
        self.cur_token.clone()
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = vec![];

        while !self.lexer.is_empty() {
            let cur_token = self.next_token().unwrap();
            match cur_token {
                Token::Identifier(s) => {
                    let next_token = self.next_token().expect("invalid statement");
                    if next_token == Token::Colon {
                        statements.push(self.parse_rule(s));
                    }
                }
                Token::Colon => todo!(),
                Token::SemiColon => todo!(),
                Token::Tab => todo!(),
                Token::NewLine => continue,
                Token::UserVariable(_) => todo!(),
                Token::ConfigVariable(_) => todo!(),
            }
        }

        Program {
            statements,
            symbol_table: HashMap::default(),
        }
    }

    fn consume_identifier(&mut self) -> String {
        let t = self.cur_token().expect("consume error");
        match t {
            Token::Identifier(s) => s,
            _ => panic!("unexpected token {:?}, expected Token::Identifier", t),
        }
    }
    fn consume_tab(&mut self) {
        let t = self.cur_token().expect("consume error");
        match t {
            Token::Tab => {}
            _ => panic!("unexpected token {:?}, expected Token::Identifier", t),
        }
    }

    fn parse_rule(&mut self, target: String) -> Statement {
        let mut prerequisite = vec![];
        loop {
            if let Some(tok) = self.next_token() {
                if tok == Token::NewLine {
                    break;
                }
                prerequisite.push(self.consume_identifier());
            } else {
                break;
            }
        }
        let mut commands = vec![];
        self.next_token();
        loop {
            if self.cur_token().is_some() && self.cur_token().unwrap() == Token::Tab {
                self.consume_tab();
                commands.push(self.lexer.next_command());
                self.next_token();
            } else {
                break;
            }
        }

        Box::new(RuleStmt {
            target,
            prerequisite,
            commands,
        })
    }
}

impl Program {
    pub fn walk_to_node(&self, visitor: Visitor) -> Node {
        let mut node = Node::default();
        for stmt in &self.statements {
            node = stmt.walk_to_node(visitor);
        }
        node
    }
    pub fn walk_to_string(&self, visitor: Visitor) -> String {
        let mut s = "".to_string();
        for stmt in &self.statements {
            s.push_str(&stmt.walk_to_string(visitor));
            s.push('\n');
        }
        s
    }
}
