use core::panic;
use std::collections::HashMap;

use crate::lexer::{Command, Lexer, Token};

pub fn walk_stmt<V: Visitor>(visitor: &mut V, stmt: &Statement) {
    match stmt {
        Statement::RuleStmt(s) => visitor.visit_rule(s),
        Statement::AssignStmt(s) => visitor.visit_assign(s),
        Statement::DirectiveStmt(s) => visitor.visit_directive(s),
    };
}
pub trait Visitor: Sized {
    fn visit_rule(&mut self, t: &RuleStmt);
    fn visit_assign(&mut self, t: &AssignStmt);
    fn visit_directive(&mut self, t: &DirectiveStmt);
    fn visit_program(&mut self, t: &Program) {
        for stmt in &t.statements {
            walk_stmt(self, stmt);
        }
    }
}

pub enum Statement {
    RuleStmt(RuleStmt),
    AssignStmt(AssignStmt),
    DirectiveStmt(DirectiveStmt),
}

pub struct RuleStmt {
    pub target: String,
    pub prerequisite: Vec<String>,
    pub commands: Vec<Command>,
}
pub struct AssignStmt {}
pub struct DirectiveStmt {}

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

        Statement::RuleStmt(RuleStmt {
            target,
            prerequisite,
            commands,
        })
    }
}
