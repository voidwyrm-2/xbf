use std::collections::HashMap;

use crate::{
    common::XBFError,
    lexer::{Token, TokenType},
};

pub fn match_brackets(tokens: &Vec<Token>) -> Result<HashMap<usize, usize>, XBFError> {
    let mut matches: HashMap<usize, usize> = HashMap::new();

    let mut stack: Vec<(usize, &Token)> = Vec::new();

    for (i, t) in tokens.iter().enumerate() {
        if *t == TokenType::BracketOpen {
            stack.push((i, t));
        } else if *t == TokenType::BracketClose {
            if let Some(opening) = stack.pop() {
                matches.insert(i, opening.0);
                matches.insert(opening.0, i);
            } else {
                let msg = format!("{} mismatched ']'", t.err());
                return Err(XBFError::from(msg));
            }
        }
    }

    if let Some(top) = stack.pop() {
        let msg = format!("{} mismatched '['", top.1.err());
        return Err(XBFError::from(msg));
    }

    Ok(matches)
}
