use std::io::{self, Cursor, Read, Seek};

pub struct Lexer {
    cursor: Cursor<String>,
}

pub enum Token {
    Identifier(String),
    Colon,
    SemiColon,
    Tab,
    UserVariable(String),   //@(var)
    ConfigVariable(String), //.PRECIPEPREFIX
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
            self.cursor.seek(io::SeekFrom::Current(-1)).unwrap();
        }
        res
    }
    fn read_char(&mut self) -> Option<char> {
        let mut buf = [0; 1];
        if self.cursor.read(&mut buf).is_err() {
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

    fn read_word(&mut self, end: &str) -> String {
        let mut s = "".to_string();
        while let Some(c) = self.read_char() {
            if end.contains(c) {
                break;
            }
            s.push(c);
        }
        s
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
            Some('$') => {
                if self.expected('(') {
                    let s = self.read_word(")");
                    Some(Token::UserVariable(s))
                } else {
                    // TODO: wildcard function
                    unimplemented!()
                }
            }
            Some('.') => {
                let s = self.read_word("\t\n\r");
                Some(Token::ConfigVariable(s))
            }
            Some(c) => {
                let mut s = "".to_string();
                s.push(c);
                s.push_str(&self.read_word(" \t\n\r"));
                Some(Token::Identifier(s))
            }
        }
    }
}
