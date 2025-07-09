use std::fmt::{Debug, Display};

use crate::common::try_index;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Inc(usize),
    Dec(usize),
    Left(usize),
    Right(usize),
    BracketOpen,
    BracketClose,
    PutChar,
    GetChar,
}

pub struct Token {
    typ: TokenType,
    col: usize,
    ln: usize,
}

impl Token {
    pub fn new(typ: TokenType, col: usize, ln: usize) -> Token {
        Token {
            typ: typ,
            col: col,
            ln: ln,
        }
    }

    pub fn get_typ(&self) -> &TokenType {
        &self.typ
    }

    pub fn err(&self) -> String {
        format!("Error on line {}, col {}:", self.ln, self.col)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}, {}, {}>", self.typ, self.col, self.ln)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.typ == *other
    }

    fn ne(&self, other: &TokenType) -> bool {
        !(self == other)
    }
}

pub struct Lexer {
    text: Vec<u8>,
    idx: usize,
    col: usize,
    ln: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.into(),
            idx: 0,
            col: 1,
            ln: 1,
        }
    }

    fn mct(char: &u8, size: usize) -> TokenType {
        match char {
            b'+' => TokenType::Inc(size),
            b'-' => TokenType::Dec(size),
            b'<' => TokenType::Left(size),
            b'>' => TokenType::Right(size),
            _ => unreachable!(),
        }
    }

    fn adv(&mut self) {
        self.idx += 1;
        self.col += 1;

        if try_index(&self.text, self.idx).is_some_and(|ch| *ch == b'\n') {
            self.ln += 1;
            self.col = 1;
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let text_clone = self.text.clone();

        while let Some(ch) = try_index(&text_clone, self.idx) {
            match ch {
                b'+' | b'-' | b'<' | b'>' => {
                    let mut size: usize = 0;

                    while try_index(&text_clone, self.idx).is_some_and(|c| c == ch) {
                        size += 1;
                        self.adv();
                    }

                    tokens.push(Token::new(Lexer::mct(ch, size), self.col, self.ln));
                }
                b'[' => {
                    tokens.push(Token::new(TokenType::BracketOpen, self.col, self.ln));
                    self.adv();
                }
                b']' => {
                    tokens.push(Token::new(TokenType::BracketClose, self.col, self.ln));
                    self.adv();
                }
                b'.' => {
                    tokens.push(Token::new(TokenType::PutChar, self.col, self.ln));
                    self.adv();
                }
                b',' => {
                    tokens.push(Token::new(TokenType::GetChar, self.col, self.ln));
                    self.adv();
                }
                _ => self.adv(),
            }
        }

        tokens
    }
}
