use std::io::{self, Cursor, Read, Seek};

pub struct Lexer {
    cursor: Cursor<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Identifier(String),
    Colon,
    SemiColon,
    Tab,
    NewLine,
    UserVariable(String),   //@(var)
    ConfigVariable(String), //.PRECIPEPREFIX
}

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self {
            cursor: Cursor::new(contents),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        let res = self.read_char();
        if res.is_some() {
            self.unread();
        }
        res
    }
    fn unread(&mut self) {
        self.cursor.seek(io::SeekFrom::Current(-1)).unwrap();
    }
    fn finish(&self) -> bool {
        self.cursor.position() as usize == self.cursor.get_ref().len()
    }
    fn read_char(&mut self) -> Option<char> {
        let mut buf = [0; 1];
        if self.finish() || self.cursor.read(&mut buf).is_err() {
            return None;
        }
        Some(u8::from_le_bytes(buf) as char)
    }

    fn expected(&mut self, expected: char) -> bool {
        match self.peek_char() {
            Some(c) => {
                if c == expected {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn read_word_until(&mut self, end: &str) -> String {
        let mut s = "".to_string();
        while let Some(c) = self.read_char() {
            if end.contains(c) {
                self.unread();
                return s;
            }
            s.push(c);
        }
        s
    }

    fn read_word(&mut self) -> String {
        self.read_word_until(" \t\n\r")
    }

    fn read_line(&mut self) -> String {
        self.read_word_until("\n")
    }

    pub fn next_command(&mut self) -> Command {
        let command = self.read_word();
        let line = self.read_line();
        let args = line.split(' ').map(|s| s.to_string()).collect();
        Command { command, args }
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek_char().is_none()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_char() {
            None => None,
            Some(':') => Some(Token::Colon),
            Some(';') => Some(Token::SemiColon),
            Some('\t') => Some(Token::Tab),
            Some(' ') => self.next(),
            Some('\n') => Some(Token::NewLine),
            Some('$') => {
                if self.expected('(') {
                    let s = self.read_word_until(")");
                    Some(Token::UserVariable(s))
                } else {
                    // TODO: wildcard function
                    unimplemented!()
                }
            }
            Some('.') => {
                let s = self.read_word();
                Some(Token::ConfigVariable(s))
            }
            Some(c) => {
                let mut s = "".to_string();
                s.push(c);
                s.push_str(&self.read_word());
                Some(Token::Identifier(s))
            }
        }
    }
}
