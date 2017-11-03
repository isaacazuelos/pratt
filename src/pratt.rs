use std::collections::HashMap;

use token::{Token, TokenKind};

// type BindingPower = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Prefix(TokenKind, Box<AST>),
    Value(i16),
}

type PrefixRule = fn(&mut Parser, Token) -> AST; 
// type InfixFule = fn(&mut Parser, AST, Token) -> AST;

#[derive(Default)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
    prefix_rules: HashMap<TokenKind, PrefixRule>,
    // infix_fules: HashMap<TokenKind, InfixFule>,
}

impl Parser {
    pub fn load_input(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.index = 0
    }
    
    pub fn register_prefix(&mut self, kind: TokenKind, rule: PrefixRule) {
        self.prefix_rules.insert(kind, rule);
    }

    pub fn consume(&mut self) -> Token {
        self.index += 1;
        self.tokens[self.index - 1]
    }

    pub fn expression(&mut self) -> AST {
        let token = self.consume();

        self.prefix_rules[&token.kind()](self, token)
    }
}

impl Parser {
    pub fn generic_prefix(&mut self, token: Token) -> AST {
        let expression = self.expression();
        AST::Prefix(token.kind(), Box::new(expression))
    }
}

// example

pub fn sample_parser() -> Parser {
    let mut parser = Parser::default();
    {
        use token::TokenKind::*;
        parser.register_prefix(Value, |_, t| {
            match t {
                Token::Digit(u) => AST::Value(u),
                _ => panic!("rule for digit called on invalid token: {:?}", t)
            }
        });
        parser.register_prefix(Plus, Parser::generic_prefix);
        parser.register_prefix(Hyphen, Parser::generic_prefix);
        parser.register_prefix(Bang, Parser::generic_prefix);
    }
    parser