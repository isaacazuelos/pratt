use std::char;

#[derive(Eq, Copy, Clone, Debug, PartialEq, Hash)]
pub enum TokenKind {
    Value,
    Plus, // pre/in
    Times, // in
    Hyphen, // pre/in 
    Bang,  // pre
}

#[derive(Eq, Copy, Clone, Debug, PartialEq, Hash)]
pub enum Token {
    Op(TokenKind),
    Digit(i16),
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        use self::TokenKind::*;
        use self::Token::*;
        match self {
            &Op(k) => k,
            &Digit(_) => Value,
        }
    } 
}

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut result = Vec::new();
    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }
        use self::TokenKind::*;
        use self::Token::*;
        match c {
            '+' => {
                result.push(Op(Plus));
            }
            '*' => {
                result.push(Op(Times));
            }
            '-' => {
                result.push(Op(Hyphen));
            }
            '!' => {
                result.push(Op(Bang));
            }
            c => {
                match char_to_i16(c) {
                    None => return None,
                    Some(u) => result.push(Digit(u)),
                }
            }
        }
    }
    Some(result)
}

fn char_to_i16(c: char) -> Option<i16> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }

}
