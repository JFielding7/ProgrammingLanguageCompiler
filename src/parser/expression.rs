use crate::ast::ast_tree::ASTNode;
use crate::error::compiler_error::Result;
use crate::error::compiler_error::CompilerError::UnmatchedParen;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::Token;
use crate::parser::sub_expression::SubExpressionParser;

pub struct ExpressionParser {
    tokens: Vec<Token>,
    paren_matches: Vec<usize>,
}

impl ExpressionParser {
    pub fn parse(tokens: Vec<Token>) -> Result<ASTNode> {
        let parser = Self::new(tokens)?;

        SubExpressionParser::new(
            &parser.tokens,
            &parser.paren_matches,
        ).parse()
    }
    
    fn new(
        tokens: Vec<Token>,
    ) -> Result<Self> {
        let paren_matches = Self::match_parens(&tokens)?;

        Ok(Self {
            tokens,
            paren_matches,
        })
    }

    fn match_parens(expression: &Vec<Token>) -> Result<Vec<usize>> {

        let mut paren_matches = vec![0; expression.len()];
        let mut open_parens = Vec::new();

        for (i, token) in expression.iter().enumerate() {
            if let OpenParen = token.token_type {
                open_parens.push(i);
            } else if let CloseParen = token.token_type {
                if let Some(j) = open_parens.pop() {
                    paren_matches[i] = j;
                } else {
                    return Err(UnmatchedParen(token.error_info.clone()));
                }
            }
        }

        if let Some(j) = open_parens.pop() {
            return Err(UnmatchedParen(expression[j].error_info.clone()));
        }

        Ok(paren_matches)
    }
}
