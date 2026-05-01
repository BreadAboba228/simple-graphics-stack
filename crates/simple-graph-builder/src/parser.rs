use std::{iter::Peekable, ops::RangeInclusive, slice::Iter, str::Chars};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Variable,
    Plus, Minus, Multiply, Divide, LParen, RParen, Power,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: RangeInclusive<usize>,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: usize,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar { ch: char, span: RangeInclusive<usize> },
    NumberTooBig { span: RangeInclusive<usize> }
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { chars: str.chars().peekable(), pos: 0 }
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().inspect(|_| self.pos += 1)
    }

    fn single_char_token(&self, kind: TokenKind) -> Token {
        Token { kind, span: self.pos..=self.pos }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.next() {
            match ch.to_ascii_lowercase() {
                '0'..='9' => {
                    let first_pos = self.pos;
                    let mut num_str = String::from(ch);

                    while let Some(&f) = self.chars.peek() {
                        if f.is_digit(10) || f == '.' {
                            self.next();
                            num_str.push(f);
                        } else {
                            break;
                        }
                    }

                    let result = num_str.parse::<f64>();

                    match result {
                        Ok(num) => tokens.push(Token { kind: TokenKind::Number(num), span: first_pos..=self.pos }),

                        Err(_) => return Err(LexerError::NumberTooBig { span: first_pos..=self.pos })
                    }
                },

                'x' => tokens.push(self.single_char_token(TokenKind::Variable)),

                '+' => tokens.push(self.single_char_token(TokenKind::Plus)),

                '-' => tokens.push(self.single_char_token(TokenKind::Minus)),

                '*' => tokens.push(self.single_char_token(TokenKind::Multiply)),

                '/' => tokens.push(self.single_char_token(TokenKind::Divide)),

                '(' => tokens.push(self.single_char_token(TokenKind::LParen)),

                ')' => tokens.push(self.single_char_token(TokenKind::RParen)),

                '^' => tokens.push(self.single_char_token(TokenKind::Power)),

                ' ' => continue,

                _ => return Err(LexerError::UnexpectedChar { ch, span: self.pos..=self.pos })
            }
        }

        Ok(tokens)
    }
}

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current_token: Option<&'a Token>,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
    ExpectedToken(TokenKind),
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens: tokens.iter(), current_token: None }
    }

    fn next(&mut self) -> Option<&'a Token> {
        self.current_token = self.tokens.next();
        self.current_token
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParserError> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut node = self.parse_term()?;

        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::Plus => {
                    self.next();

                    node = Box::new(Expr::Add(node, self.parse_term()?));
                }

                TokenKind::Minus => {
                    self.next();

                    node = Box::new(Expr::Sub(node, self.parse_term()?))
                }

                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut node = self.parse_power()?;

        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::Multiply => {
                    self.next();

                    node = Box::new(Expr::Mul(node, self.parse_power()?))
                },

                TokenKind::Divide => {
                    self.next();

                    node = Box::new(Expr::Div(node, self.parse_power()?))
                },

                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_power(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut node = self.parse_factor()?;

        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::Power => {
                    self.next();

                    node = Box::new(Expr::Power(node, self.parse_factor()?))
                },

                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, ParserError> {
        if let Some(token) = self.current_token {
            match token.kind {
                TokenKind::Number(n) => {
                    self.next();

                    Ok(Box::new(Expr::Number(n)))
                },

                TokenKind::Variable => {
                    self.next();

                    Ok(Box::new(Expr::Variable))
                }

                TokenKind::LParen => {
                    self.next();

                    let node = self.parse_expr();

                    if let Some(token) = self.next() && token.kind == TokenKind::RParen {
                        node
                    } else {
                        Err(ParserError::ExpectedToken(TokenKind::RParen))
                    }
                },

                _ => Err(ParserError::UnexpectedToken(token.clone())),
            }
        } else {
            Err(ParserError::ExpectedToken(TokenKind::Variable))
        }
    }
}

pub enum Expr {
    Number(f64),
    Variable,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Expr::Number(n) => *n,

            Expr::Variable => x,

            Expr::Add(a, b) => a.eval(x) + b.eval(x),

            Expr::Sub(a, b) => a.eval(x) - b.eval(x),

            Expr::Mul(a, b) => a.eval(x) * b.eval(x),

            Expr::Div(a, b) => a.eval(x) / b.eval(x),

            Expr::Power(a, b) => a.eval(x).powf(b.eval(x)),
        }
    }
}
