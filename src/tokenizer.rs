use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::iter::{Iterator, Peekable};
use std::str::Chars;

#[derive(PartialEq)]
pub enum Token {
    Invalid,

    // Values
    Boolean(bool),
    Integer(i32),
    Name(String),

    // Keywords
    And,
    Array,
    If,
    Let,
    Not,
    Or,
    Print,
    While,

    // Symbols
    Asterisk,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    Colon,
    Dot,
    EqualSign,
    Minus,
    ParenLeft,
    ParenRight,
    Plus,
    Semicolon,
    Slash,

    // Operators
    Assign,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut map = HashMap::new();
        map.insert("and", Token::And);
        map.insert("array", Token::Array);
        map.insert("if", Token::If);
        map.insert("let", Token::Let);
        map.insert("not", Token::Not);
        map.insert("or", Token::Or);
        map.insert("print", Token::Print);
        map.insert("while", Token::While);
        map
    };
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    len: usize,
    pos: u32,
    line: u32,
    col: u32,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        let mut iter = input.chars().peekable();
        Tokenizer {
            input: iter,
            len: input.len(),
            pos: 0,
            line: 1,
            col: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume_whitespace(&mut self) {
        if let Some(&c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\n' => {
                    while let Some(&c) = self.peek_char() {
                        match c {
                            ' ' | '\t' | '\n' => {
                                if c == '\n' {
                                    self.line += 1
                                };
                                self.pos += 1;
                                self.next_char();
                            }
                            _ => break,
                        }
                    }
                }
                _ => return,
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.consume_whitespace();
        if let Some(c) = self.next_char() {
            let result = match c {
                // Symbols
                '*' => Token::Asterisk,
                '{' => Token::BraceLeft,
                '}' => Token::BraceRight,
                '[' => Token::BracketLeft,
                ']' => Token::BracketRight,
                ':' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.next_char();
                            Token::Assign
                        }
                        _ => Token::Colon,
                    }
                }
                '.' => Token::Dot,
                '=' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.next_char();
                            Token::Eq
                        }
                        _ => Token::EqualSign,
                    }
                }
                '-' => Token::Minus,
                '(' => Token::ParenLeft,
                ')' => Token::ParenRight,
                '+' => Token::Plus,
                ';' => Token::Semicolon,
                '/' => Token::Slash,
                '>' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.next_char();
                            Token::Ge
                        }
                        _ => Token::Gt,
                    }
                }
                '<' => {
                    match self.peek_char() {
                        Some(&'=') => {
                            self.next_char();
                            Token::Le
                        }
                        Some(&'>') => {
                            self.next_char();
                            Token::Ne
                        }
                        _ => Token::Lt,
                    }
                }

                // Numbers
                '0'...'9' => {
                    let mut s = c.to_string();
                    while let Some(&c) = self.peek_char() {
                        match c {
                            '0'...'9' => {
                                s.push(c);
                            }
                            _ => break,
                        };
                        self.next_char();
                    }
                    let num = s.parse::<i32>().unwrap();
                    Token::Integer(num)
                }

                // Alphanums
                'a'...'z' | 'A'...'Z' => {
                    let mut s = c.to_string();
                    while let Some(&c) = self.peek_char() {
                        match c {
                            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                                s.push(c);
                            }
                            _ => break,
                        };
                        self.next_char();
                    }
                    match KEYWORDS.get(s.as_str()) {
                        Some(&Token::And) => Token::And,
                        Some(&Token::Array) => Token::Array,
                        Some(&Token::If) => Token::If,
                        Some(&Token::Let) => Token::Let,
                        Some(&Token::Not) => Token::Not,
                        Some(&Token::Or) => Token::Or,
                        Some(&Token::Print) => Token::Print,
                        Some(&Token::While) => Token::While,
                        _ => Token::Name(s),
                    }
                }

                // Anything else
                _ => Token::Invalid,
            };
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn consume_whitespace() {
        use super::Tokenizer;
        let mut t = Tokenizer::new("t \t\na");

        t.consume_whitespace();
        assert!(t.peek_char() == Some(&'t'));

        t.next_char();
        t.consume_whitespace();
        assert!(t.peek_char() == Some(&'a'));

        t.next_char();
        t.consume_whitespace();
        assert!(t.peek_char() == None);
    }

    #[test]
    fn symbol_tokens() {
        use super::{Token, Tokenizer};
        let mut t = Tokenizer::new("+-*/::=<<=");

        assert!(t.next() == Some(Token::Plus));
        assert!(t.next() == Some(Token::Minus));
        assert!(t.next() == Some(Token::Asterisk));
        assert!(t.next() == Some(Token::Slash));
        assert!(t.next() == Some(Token::Colon));
        assert!(t.next() == Some(Token::Assign));
        assert!(t.next() == Some(Token::Lt));
        assert!(t.next() == Some(Token::Le));
        assert!(t.next() == None);
    }

    #[test]
    fn int_tokens() {
        use super::{Token, Tokenizer};
        let mut t = Tokenizer::new("1 2 3 123 987");

        assert!(t.next() == Some(Token::Integer(1)));
        assert!(t.next() == Some(Token::Integer(2)));
        assert!(t.next() == Some(Token::Integer(3)));
        assert!(t.next() == Some(Token::Integer(123)));
        assert!(t.next() == Some(Token::Integer(987)));
        assert!(t.next() == None);
    }

    #[test]
    fn keyword_tokens() {
        use super::{Token, Tokenizer};
        let mut t = Tokenizer::new("and array if let not or print while");

        assert!(t.next() == Some(Token::And));
        assert!(t.next() == Some(Token::Array));
        assert!(t.next() == Some(Token::If));
        assert!(t.next() == Some(Token::Let));
        assert!(t.next() == Some(Token::Not));
        assert!(t.next() == Some(Token::Or));
        assert!(t.next() == Some(Token::Print));
        assert!(t.next() == Some(Token::While));
        assert!(t.next() == None);
    }

    #[test]
    fn name_tokens() {
        use super::{Token, Tokenizer};
        let mut t = Tokenizer::new("and xxx if If");

        assert!(t.next() == Some(Token::And));
        assert!(t.next() == Some(Token::Name("xxx".to_string())));
        assert!(t.next() == Some(Token::If));
        assert!(t.next() == Some(Token::Name("If".to_string())));
        assert!(t.next() == None);
    }
}
