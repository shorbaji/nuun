use crate::lexer::{DLexer, Token};
use crate::object::{Atom, Sexp};
use crate::error::DalError;

pub struct Parser {
    tokens: DLexer,
}

impl Parser {
    pub fn new(code: &str) -> Self {

        Self {
            tokens: DLexer::new(code),
        }
    }

    fn peek(&mut self) -> Option<Result<Token, ()>> {
        self.tokens.peek()
    }

    fn expect_token(&mut self, expected: Token) -> Result<Token, DalError> {
        self.peek()
        .ok_or(DalError::ParserError("expected something".to_string()))
        .and_then(|result|
            result
            .map_err(|_| DalError::LexerError)
            .and_then(|token| {
                if token == expected {
                    Ok(token)
                } else {
                    Err(DalError::ParserError(format!("expected {:?}", expected)))
                }
            })
        )
        .inspect(|_| {self.tokens.next(); })
    }

    fn paren_left(&mut self) -> Result<(), DalError> {
        self.expect_token(Token::ParenLeft)
        .map(|_| ())
    }

    fn paren_right(&mut self) -> Result<(), DalError> {
        self.expect_token(Token::ParenRight)
        .map(|_| ())
    }


    fn true_(&mut self) -> Result<Sexp, DalError> {
        self.expect_token(Token::Boolean(true))
        .map(|_| Sexp::Atom(Atom::Bool(true)))
    }

    fn false_(&mut self) -> Result<Sexp, DalError> {
        self.expect_token(Token::Boolean(false))
        .map(|_| Sexp::Atom(Atom::Bool(false)))
    }

    fn boolean(&mut self) -> Result<Sexp, DalError> {
        self.true_()
        .or_else(|_| self.false_())
    }

    fn number(&mut self) -> Result<Sexp, DalError> {
        self.tokens.peek()
        .ok_or(DalError::ParserError("expected number".to_string()))
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                match token {
                    Token::Number(n) => Ok(Sexp::Atom(Atom::Number(n))),
                    _ => Err(DalError::ParserError("expected number".to_string())),
                }
            })
        )
    }

    fn character(&mut self) -> Result<Sexp, DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                match token {
                    Token::Char(c) => Ok(Sexp::Atom(Atom::Char(c))),
                    _ => Err(DalError::ParserError("expected character".to_string())),
                }
            })
        )
    }

    fn string(&mut self) -> Result<Sexp, DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                match token {
                    Token::String(s) => Ok(Sexp::Atom(Atom::String(s))),
                    _ => Err(DalError::ParserError("expected string".to_string())),
                }
            })
        )
    }

    fn symbol(&mut self) -> Result<Sexp, DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                match token {
                    Token::Identifier(s) => Ok(Sexp::Atom(Atom::Symbol(s))),
                    _ => Err(DalError::ParserError("expected symbol".to_string())),
                }
            })
        )
    }

    fn simple(&mut self) -> Result<Sexp, DalError> {
        self.boolean()?
        .or_else(|| self.number()?)
        .or_else(|| self.character()?)
        .or_else(|| self.string()?)
        .or_else(|| self.symbol()?)
    }

    fn get(&mut self, expected: Token) -> Result<(), DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                if token == expected {
                    Ok(())
                } else {
                    Err(DalError::ParserError(format!("expected {:?}", expected)))
                }
            })
        )
        .unwrap_or(Err(DalError::ParserError("expected something".to_string())))
    }

    fn paren_left(&mut self) -> Result<(), DalError> {
        self.get(Token::ParenLeft)
    }

    fn one_or_more<F, T>(&mut self, mut f: F) -> Result<(), DalError>
    where
        F: FnMut(&mut Self) -> Option<Result<T, DalError>>
    {
        f(self).ok_or(DalError::ParserError("expected one or more".to_string()))?;

        while let Some(result) = f(self) {
            match result {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn pair(&mut self) -> Result<Sexp, DalError> {
        self.paren_left()?;

        let car = self.sexp()?;
        let cdr = self.cdr()?;

        Ok(Sexp::Pair(Box::new(car), Box::new(cdr)))

    }

    fn compound(&mut self) -> Result<Sexp, DalError> {
        self.tokens.peek()
        .map(|result|
            result
            .map_err(|e| DalError::LexerError)
            .and_then(|token| {
                match token {
                    Token::ParenLeft => self.list(),
                    Token::HashOpen => self.vector(),
                    Token::HashU8Open => self.bytevector(),
                    _ => Err(DalError::ParserError("expected compound expression".to_string())),
                }
            })
        )
    }

    fn sexp(&mut self) -> Result<Sexp, DalError> {
        self.simple()
        .or_else(|| self.compound())
    }
}

impl<'a> std::iter::Iterator for Parser<'a> {
    type Item = Result<Sexp, DalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens
        .peek()
        .and_then(|_|
            Some(self.sexp()))
        
    }
}

