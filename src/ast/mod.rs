pub mod expressions;
pub mod module;
pub mod statements;
pub mod types;

use miette::SourceSpan;

pub type Span = SourceSpan;

/// The metadata wrapper type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct M<T> {
    pub span: Span,
    pub value: T
}

impl<T> M<T> {
    pub fn new(value: T, span: Span) -> Self {
        M { span, value }
    }

    pub fn new_range(value: T, left: Span, right: Span) -> Self {
        let left_most = left.offset();
        let right_most = right.offset() + right.len();
        let len = right_most - left_most;
        let span = Span::from((left_most, len));
        M { span, value }
    }
}

/// The metadata wrapper type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MBox<T> {
    pub span: Span,
    pub value: Box<T>
}

impl<T> MBox<T> {
    pub fn new(value: T, span: Span) -> Self {
        MBox { span, value: Box::new(value) }
    }

    pub fn new_range(value: T, left: Span, right: Span) -> Self {
        let left_most = left.offset();
        let right_most = right.offset() + right.len();
        let len = right_most - left_most;
        let span = Span::from((left_most, len));
        MBox { span, value: Box::new(value) }
    }
}

/// A place value represents a memory location
/// e.g. foobar, foobar[0], foobar[0..199]
/// Places can be dereferenced, assigned to, and operated on
/// They are used in expressions and statements
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Place {
    Identifier {
        ident: M<String>
    }
}