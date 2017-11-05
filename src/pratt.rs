use std::collections::HashMap;
use std::rc::Rc;

use token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidRule(TokenKind, TokenKind),
    EndOfStream
}

type ParseResult<T> = Result<T, ParserError>;

type Precedence = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Prefix(TokenKind, Box<AST>),
    Infix(TokenKind, Box<AST>, Box<AST>),
    Value(i16),
}

#[derive(Clone)]
pub struct PrefixRule {
    rule: Rc<Fn(&mut Parser, Token) -> ParseResult<AST>>,
}

impl PrefixRule {
    pub fn generic(precedence: Precedence) -> PrefixRule {
        PrefixRule {
            rule: Rc::new(move |parser, token| {
                let node = parser.expression(precedence)?;
                Ok(AST::Prefix(token.kind(), Box::new(node)))
            }),
        }
    }

    pub fn new(rule: Rc<Fn(&mut Parser, Token) -> ParseResult<AST>>) -> PrefixRule {
        PrefixRule { rule }
    }

    pub fn parse(&self, parser: &mut Parser, token: Token) -> ParseResult<AST> {
        (self.rule)(parser, token)
    }
}

#[derive(Clone)]
pub struct InfixRule {
    rule: fn(&mut Parser, AST, Token) -> ParseResult<AST>,
    precedence: Precedence,
}

impl InfixRule {
    pub fn new(rule: fn(&mut Parser, AST, Token) -> ParseResult<AST>, precedence: Precedence) -> Self {
        InfixRule { rule, precedence }
    }

    pub fn parse(&self, parser: &mut Parser, node: AST, token: Token) -> ParseResult<AST> {
        (self.rule)(parser, node, token)
    }
}

#[derive(Default)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
    prefix_rules: HashMap<TokenKind, PrefixRule>,
    infix_rules: HashMap<TokenKind, InfixRule>,
}

impl Parser {
    pub fn load_input(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.index = 0
    }

    pub fn register_prefix(&mut self, kind: TokenKind, rule: PrefixRule) {
        self.prefix_rules.insert(kind, rule);
    }

    pub fn register_infix(&mut self, kind: TokenKind, rule: InfixRule) {
        self.infix_rules.insert(kind, rule);
    }

    pub fn consume(&mut self) -> ParseResult<Token> {
        let token = self.look_ahead()?;
        self.index += 1;
        Ok(token)
    }

    pub fn look_ahead(&mut self) -> ParseResult<Token> {
        match self.tokens.get(self.index) {
            Some(t) => Ok(t.clone()),
            None => Err(ParserError::EndOfStream)
        }
    }

    pub fn expression(&mut self, precedence: Precedence) -> ParseResult<AST> {
        let mut token = self.consume()?;
        let prefix: PrefixRule;

        // More advanced error handling would be needed here, later.
        match self.prefix_rules.get(&token.kind()) {
            None => panic!("No rule for parsing token: {:?}", token),
            Some(p) => prefix = p.clone(),
        }

        let mut left = prefix.parse(self, token)?;

        while precedence < self.next_token_precedence() {
            token = self.consume()?;
            let infix = self.infix_rules[&token.kind()].clone();
            left = infix.parse(self, left, token)?;
        }

        Ok(left)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        self.look_ahead().and_then(|t| 
            self.infix_rules.get(&t.kind()).map(|r| r.precedence).ok_or(ParserError::EndOfStream)
        ).unwrap_or_default()
    }
}

pub fn sample_parser() -> Parser {
    let mut parser = Parser::default();
    {
        use token::TokenKind::*;
        parser.register_prefix(
            Value,
            PrefixRule::new(Rc::new(|_, t| match t {
                Token::Digit(u) => Ok(AST::Value(u)),
                _ => Err(ParserError::InvalidRule(t.kind(), Value)),
            })),
        );
        parser.register_prefix(Plus, PrefixRule::generic(10));
        parser.register_prefix(Hyphen, PrefixRule::generic(10));
        parser.register_prefix(Bang, PrefixRule::generic(5));

        parser.register_infix(Plus, InfixRule::new(|p, left, token| {
            println!("running infix plus");
            Ok(AST::Infix(token.kind(), Box::new(left), Box::new(p.expression(0)?)))
        }, 2));
    }
    parser
}