use crate::ast;
use crate::parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Basic
    Illegal(String),
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(i32),
    Float(f32),

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Lt,
    Gt,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Let,
    Fn,
    True,
    False,
    If,
    Else,
    Return,
    And,
    Or,

    // Multi char operators
    Equals,
    Differs,
    Ge,
    Le,
}

impl Token {
    fn parse_prefix_expression(&self, p: &mut parser::Parser) -> Result<ast::Expression, String> {
        // If we're here, we assume that self if one of [Bang, Minus]
        p.next_token();
        let expr = p.parse_expression(parser::Precedence::Prefix)?;

        Ok(ast::Expression::Prefix(ast::expressions::Prefix::new(
            self.clone(),
            expr,
        )))
    }
    pub fn parse_prefix(&self, p: &mut parser::Parser) -> Result<ast::Expression, String> {
        match self {
            Token::Ident(ident) => Ok(ast::Expression::Identifier(
                // ident is a &String, so we need to clone
                ast::expressions::Identifier::new(ident.clone()),
            )),
            Token::Int(i) => Ok(ast::Expression::Int(ast::expressions::Int::new(*i))),
            Token::Float(f) => Ok(ast::Expression::Float(ast::expressions::Float::new(*f))),
            Token::Bang | Token::Minus => self.parse_prefix_expression(p),
            _ => Err(format!("Unsupported prefix token {:?}", self)),
        }
    }

    pub fn parse_infix(
        &self,
        p: &mut parser::Parser,
        left: ast::Expression,
    ) -> Result<ast::Expression, String> {
        if let Some(precedence) = p.cur_precedence() {
            p.next_token();
            let right = p.parse_expression(precedence)?;
            Ok(ast::Expression::Infix(ast::expressions::Infix::new(
                self.clone(),
                left,
                right,
            )))
        } else {
            Err("Current token has no precedence".to_string())
        }
    }

    // Returns a Token's precedence
    pub fn precedence(&self) -> Option<parser::Precedence> {
        match self {
            Token::Equals => Some(parser::Precedence::Equals),
            Token::Differs => Some(parser::Precedence::Equals),
            Token::Lt => Some(parser::Precedence::LessGreater),
            Token::Gt => Some(parser::Precedence::LessGreater),
            Token::Plus => Some(parser::Precedence::Sum),
            Token::Minus => Some(parser::Precedence::Sum),
            Token::Slash => Some(parser::Precedence::Product),
            Token::Asterisk => Some(parser::Precedence::Product),
            _ => None,
        }
    }
}
