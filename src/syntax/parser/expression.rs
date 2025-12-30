use std::ops::Deref;
use crate::error_util::SourceLocation;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::{Equals, CloseParen, Identifier, IntLiteral, Minus, OpenParen, Plus, StringLiteral, Star, Slash};
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::error::SyntaxError::InvalidExpression;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use crate::syntax::error::unmatched_paren::UnmatchedParenError;
use crate::syntax::parser::statement::Statement;

const OPERATOR_GROUPS_COUNT: usize = 2;
const OPERATORS: [&[TokenType]; OPERATOR_GROUPS_COUNT] = [
    &[Equals],
    &[Plus, Minus]
];



struct ExpressionParser<'a> {
    tokens: &'a [Token],
    start: usize,
    end: usize,
    paren_matches: &'a Vec<usize>,
    start_op_group: usize,
}

impl <'a> ExpressionParser<'a> {

    pub fn new(tokens: &'a [Token], start: usize, paren_matches: &'a Vec<usize>) -> Self {
        Self {
            tokens,
            start,
            end: tokens.len(),
            paren_matches,
            start_op_group: 0
        }
    }

    fn sub_expression(&'a self, start: usize, end: usize, start_op_group: usize) -> Self {
        Self {
            tokens: self.tokens,
            start,
            end,
            paren_matches: self.paren_matches,
            start_op_group
        }
    }

    fn remove_redundant_parens(&mut self) {
        let prev_start = self.start;

        while self.start < self.len() && self[self.start] == OpenParen && self[self.end - 1] == CloseParen {
            self.start += 1;
            self.end -= 1;
        }

        if self.start != prev_start {
            self.start_op_group = 0;
        }
    }

    pub fn parse(&mut self) -> SyntaxResult<ASTNode> {

        self.remove_redundant_parens();

        if self.start >= self.end {
            return Err(self.empty_expression_error());
        }

        if self.start + 1 == self.end {
            return parse_value(&self[self.start]);
        }

        while self.start_op_group < OPERATOR_GROUPS_COUNT {
            let start = self.start as isize;
            let mut i = self.end as isize - 1;

            while i >= start {
                let index = i  as usize;

                if in_operator_group(self.start_op_group, &self[index]) {

                    let left_node = self
                        .sub_expression(self.start, index, self.start_op_group)
                        .parse()?;

                    let right_node = self
                        .sub_expression(index + 1, self.end, self.start_op_group + 1)
                        .parse()?;

                    let op_token = &self[index];
                    return Ok(BinaryOperatorNode::new(op_token.into(), left_node, right_node).into());
                    
                } else if self[index] == CloseParen {
                    i = self.paren_matches[index] as isize;
                } else {
                    i -= 1;
                }
            }

            self.start_op_group += 1;
        }

        Err(InvalidExpression(self[self.start].location.clone()))
    }

    fn empty_expression_error(&self) -> SyntaxError {
        InvalidExpression(
            if self.start < self.len() {
                self[self.start].location.clone()
            } else {
                self[self.start - 1].location.clone()
            }
        )
    }
}

impl<'a> Deref for ExpressionParser<'a> {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        self.tokens
    }
}

fn parse_value(token: &Token) -> SyntaxResult<ASTNode> {
    let token_string = token.token_str.clone();

    match token.token_type {
        IntLiteral    => Ok(ASTNode::IntLiteral(token_string)),
        StringLiteral => Ok(ASTNode::StringLiteral(token_string)),
        Identifier    => Ok(ASTNode::Identifier(token_string)),
        _ => Err(InvalidExpression(token.location.clone()))
    }
}

fn in_operator_group(op_group: usize, token: &Token) -> bool {
    for token_type in OPERATORS[op_group] {
        if token == token_type {
            return true;
        }
    }

    false
}

fn match_parens(expression: &[Token]) -> SyntaxResult<Vec<usize>> {

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
                    CloseParen, token.location.clone()
                ).into());
            }
        }
    }

    if let Some(j) = open_parens.pop() {
        return Err(UnmatchedParenError::new(
            OpenParen, expression[j].location.clone()
        ).into());
    }

    Ok(paren_matches)
}

pub fn parse_expression(tokens: &[Token], start: usize) -> SyntaxResult<ASTNode> {

    ExpressionParser::new(
        tokens,
        start,
        &match_parens(&tokens[start..])?,
    ).parse()
}

impl From<&Token> for BinaryOperatorType {
    fn from(op_token: &Token) -> Self {
        match op_token.token_type {
            Equals => BinaryOperatorType::Assign,
            Plus => BinaryOperatorType::Add,
            Minus => BinaryOperatorType::Sub,
            Star => BinaryOperatorType::Mul,
            Slash => BinaryOperatorType::Div,
            _ => panic!("Token is not a valid binary operator"),
        }
    }
}
