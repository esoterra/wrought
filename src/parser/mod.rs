mod expressions;
mod module;
mod statements;
mod types;

use std::sync::Arc;

use crate::ast::Span;
use crate::lexer::{TokenData, Token};
use crate::ast::module::Module;

use miette::{Diagnostic, SourceSpan, NamedSource};
use thiserror::Error;

use self::module::parse_module;

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to parse")]
#[diagnostic()]
pub enum ParserError{
    Base {
        #[source_code]
        src: Arc<NamedSource>,
        #[label("Unable to parse this code")]
        span: SourceSpan,
    },
    UnexpectedToken {
        description: String,
        token: Option<Token>
    },
    EndOfInput,
    NotYetSupported {
        feature: String,
        token: Token
    }
}


#[derive(Debug, Clone)]
pub struct ParseInput {
    src: Arc<NamedSource>, 
    tokens: Vec<TokenData>,
    index: usize
}

#[derive(Debug, Clone, Copy)]
pub struct Checkpoint {
    index: usize
}

impl ParseInput {
    pub fn new(src: Arc<NamedSource>, tokens: Vec<TokenData>) -> Self {
        ParseInput {
            src,
            tokens,
            index: 0
        }
    }

    pub fn unsupported_error(&self, feature: &str) -> ParserError {
        ParserError::NotYetSupported {
            feature: feature.to_string(),
            token: self.tokens[self.index].token.clone()
        }
    }

    pub fn unexpected_token(&self, description: &str) -> ParserError {
        ParserError::UnexpectedToken {
            description: description.to_string(),
            token: self.tokens.get(self.index).map(|t| t.token.clone())
        }
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint { index: self.index }
    }

    pub fn restore(&mut self, checkpoint: Checkpoint) {
        self.index = checkpoint.index
    }

    pub fn get_source(&self) -> Arc<NamedSource> {
        self.src.clone()
    }

    pub fn done(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn peek(&mut self) -> Result<&TokenData, ParserError> {
        self.tokens.get(self.index).ok_or(ParserError::EndOfInput)
    }

    pub fn next(&mut self) -> Result<&TokenData, ParserError> {
        let result = self.tokens.get(self.index);
        self.index += 1;
        result.ok_or(ParserError::EndOfInput)
    }

    pub fn assert_next(&mut self, token: Token, description: &str) -> Result<Span, ParserError> {
        let next = self.next()?;
        if next.token == token {
            Ok(next.span.clone())
        } else {
            Err(self.unexpected_token(description))
        }
    }

    pub fn next_if(&mut self, token: Token) -> Option<Span> {
        {
            let next = self.peek().ok()?;
            if next.token != token {
                return None;
            }
        }
        Some(self.next().ok()?.span.clone())
    }

    pub fn has(&self, num: usize) -> bool {
        self.index + num <= self.tokens.len()
    }

    pub fn slice_next(&mut self, num: usize) -> Result<&[TokenData], ParserError> {
        if self.has(num) {
            let result = &self.tokens[self.index..self.index+num];
            self.index += num;
            Ok(result)
        } else {
            Err(ParserError::EndOfInput)
        }
    }
}


pub fn parse(src: Arc<NamedSource>, tokens: Vec<TokenData>) -> Result<Module, ParserError> {
    let mut parse_input = ParseInput::new(src, tokens);
    parse_module(&mut parse_input)
}


#[cfg(test)]
mod tests {
    
    use std::sync::Arc;
    use miette::NamedSource;

    use crate::{
        lexer::tokenize,
        ast::Span,
        parser::ParseInput
    };

    pub fn make_input(source: &str) -> ParseInput {
        let src = Arc::new(NamedSource::new("test", source.to_string()));
        let tokens = tokenize(src.clone(), source.to_string()).unwrap();
        ParseInput::new(src, tokens)
    }

    pub fn make_span(start: usize, len: usize) -> Span {
        Span::new(start.into(), len.into())
    }
}
