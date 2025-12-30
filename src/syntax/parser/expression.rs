use crate::lexer::token::Token;
use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::error::unmatched_paren::UnmatchedParenError;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::sub_expression::SubExpressionParser;

pub struct ExpressionParser<'a> {
    tokens: &'a [Token],
    paren_matches: Vec<usize>,
}

impl<'a> ExpressionParser<'a> {
    pub fn parse(statement: &'a Statement) -> SyntaxResult<ASTNode> {

        let parser = Self::new(&statement.tokens[Statement::INDEX_AFTER_INDENT..])?;

        SubExpressionParser::new(
            &parser.tokens,
            &parser.paren_matches,
        ).parse()
    }
    
    fn new(tokens: &'a [Token]) -> SyntaxResult<Self> {

        let paren_matches = Self::match_parens(tokens)?;

        Ok(Self {
            tokens,
            paren_matches,
        })
    }

    fn match_parens(expression: &'a [Token]) -> SyntaxResult<Vec<usize>> {

        let mut paren_matches = vec![0; expression.len()];
        let mut open_parens = Vec::new();

        for (i, token) in expression.iter().enumerate() {
            if let OpenParen = token.token_type {
                open_parens.push(i);
            } else if let CloseParen = token.token_type {
                if let Some(j) = open_parens.pop() {
                    paren_matches[i] = j;
                } else {
                    return Err(UnmatchedParenError::new(
                        CloseParen, token.error_location.clone()
                    ).into());
                }
            }
        }

        if let Some(j) = open_parens.pop() {
            return Err(UnmatchedParenError::new(
                OpenParen, expression[j].error_location.clone()
            ).into());
        }

        Ok(paren_matches)
    }
}
