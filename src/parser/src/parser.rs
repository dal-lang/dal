use lexer::{filter_comments, filter_non_significant, filter_whitespace, tokenize, Token};
use span::Span;

use crate::*;
use dummy::*;

/// The parser context.
pub struct ParserContext<'a> {
    pub source: &'a str,
    pub index: usize,
    pub tokens: Vec<Token>,
}

impl<'a> ParserContext<'a> {
    /// Advance to the next token.
    pub fn next(&mut self) {
        self.index += 1;
    }

    /// Get current token.
    pub fn token(&self) -> Token {
        self.tokens[self.index]
    }

    /// Get current span.
    pub fn span(&self) -> Span {
        self.token().span
    }
}

/// Create new parser context.
pub fn new_parser(input: &str) -> ParserContext {
    let tokens: Vec<Token> = tokenize(input).collect();
    ParserContext { source: input, index: 0, tokens }
}

/// The filter option for the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOption {
    /// Filter out whitespace tokens.
    Whitespace,
    /// Filter out comments.
    Comment,
    /// Filter out non-significant tokens.
    NonSignificant,
}

/// Create new parser with filtered tokens.
pub fn new_filtered_parser(input: &str, filter: FilterOption) -> ParserContext {
    let tokens: Vec<Token> = match filter {
        FilterOption::Whitespace => filter_whitespace(tokenize(input)).collect(),
        FilterOption::Comment => filter_comments(tokenize(input)).collect(),
        FilterOption::NonSignificant => filter_non_significant(tokenize(input)).collect(),
    };
    ParserContext { source: input, index: 0, tokens }
}

/// Parse root node.
pub fn parse_root(ctx: &mut ParserContext) -> ast::Root {
    let mut root = ast::Root::dummy();
    root.items = parse_items(ctx);

    if ctx.index < ctx.tokens.len() - 1 {
        parser_error!(ctx.span(), "unexpected token");
    }

    root
}
