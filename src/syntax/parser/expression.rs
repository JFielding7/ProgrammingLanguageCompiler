use crate::lexer::token::Token;
use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::binary_operator_node::BinaryOperatorType::*;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::error::unmatched_paren::UnmatchedParenError;
use crate::syntax::error::SyntaxError::InvalidExpression;
use crate::syntax::error::{SyntaxError, SyntaxResult};
use std::ops::Deref;
use crate::syntax::ast::unary_operator_node::{UnaryOperatorNode, UnaryOperatorType};
use crate::syntax::ast::unary_operator_node::UnaryOperatorType::Neg;

struct ExpressionParser<'a> {
    tokens: &'a [Token],
    curr: usize,
    end: usize,
    paren_matches: &'a Vec<usize>,
}

impl <'a> ExpressionParser<'a> {

    pub fn new(tokens: &'a [Token], paren_matches: &'a Vec<usize>) -> Self {
        Self {
            tokens,
            curr: 0,
            end: tokens.len(),
            paren_matches,
        }
    }

    fn sub_expression(&'a self, curr: usize, end: usize) -> Self {
        Self {
            tokens: self.tokens,
            curr,
            end,
            paren_matches: self.paren_matches,
        }
    }

    fn parse_value(&mut self) -> SyntaxResult<ASTNode> {
        let curr = self.curr;
        self.curr += 1;

        let token = &self[curr];
        let token_string = token.token_str.clone();

        match token.token_type {
            IntLiteral    => Ok(ASTNode::IntLiteral(token_string)),
            StringLiteral => Ok(ASTNode::StringLiteral(token_string)),
            Identifier    => Ok(ASTNode::Identifier(token_string)),
            _ => Err(InvalidExpression(token.location.clone()))
        }
    }

    fn parse_paren_expression(&mut self) -> SyntaxResult<ASTNode> {
        let curr = self.curr;
        let paren_match = self.paren_matches[curr];
        self.curr = paren_match + 1;

        self.sub_expression(curr + 1, paren_match).parse(0)
    }

    fn parse_next_sub_expression(&mut self) -> SyntaxResult<ASTNode> {
        if self[self.curr] == OpenParen {
            self.parse_paren_expression()
        } else {
            self.parse_value()
        }
    }

    fn parse_unary_operator(&mut self, unary_operator_type: UnaryOperatorType) -> SyntaxResult<ASTNode> {
        self.curr += 1;

        Ok(UnaryOperatorNode::new(
            unary_operator_type, self.parse_next_sub_expression()?
        ).into())
    }

    pub fn parse(&mut self, curr_precedence: u8) -> SyntaxResult<ASTNode> {

        if self.curr >= self.end {
            return Err(self.empty_expression_error());
        }

        let mut left_node = if let Some(u) = unary_operator(&self[self.curr]) {
            self.parse_unary_operator(u)
        } else {
            self.parse_next_sub_expression()
        }?;

        while self.curr < self.end {

            if let Some((left_prec, right_prec)) = operator_precedence(&self[self.curr]) {

                if left_prec < curr_precedence {
                    return Ok(left_node)
                }

                let op_token_index = self.curr;
                self.curr += 1;
                let right_node = self.parse(right_prec)?;

                let op_token = &self[op_token_index];
                left_node = BinaryOperatorNode::new(op_token.into(), left_node, right_node).into();

            } else {
                return Err(InvalidExpression(self[self.curr].location.clone()));
            }
        }

        Ok(left_node)
    }

    fn empty_expression_error(&self) -> SyntaxError {
        // TODO: fix if overall expr is empty
        InvalidExpression(
            if self.curr < self.len() {
                self[self.curr].location.clone()
            } else {
                self[self.curr - 1].location.clone()
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

fn match_parens(expression: &[Token]) -> SyntaxResult<Vec<usize>> {

    let mut paren_matches = vec![0; expression.len()];
    let mut open_parens = Vec::new();

    for (i, token) in expression.iter().enumerate() {
        if let OpenParen = token.token_type {
            open_parens.push(i);
        } else if let CloseParen = token.token_type {
            if let Some(j) = open_parens.pop() {
                paren_matches[j] = i;
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

pub fn parse_expression(tokens: &[Token]) -> SyntaxResult<ASTNode> {

    ExpressionParser::new(
        tokens,
        &match_parens(tokens)?,
    ).parse(0)
}

fn operator_precedence(op: &Token) -> Option<(u8, u8)> {
    match op.token_type {
        Equals => Some((1, 0)),
        Plus | Minus => Some((3, 4)),
        Star | Slash | Percent => Some((5, 6)),
        _ => None,
    }
}

fn unary_operator(op: &Token) -> Option<UnaryOperatorType> {
    match op.token_type {
        Minus => Some(Neg),
        _ => None,
    }
}

impl From<&Token> for BinaryOperatorType {
    fn from(op_token: &Token) -> Self {
        match op_token.token_type {
            Equals => Assign,
            Plus => Add,
            Minus => Sub,
            Star => Mul,
            Slash => Div,
            Percent => Mod,
            _ => panic!("Token is not a valid binary operator"),
        }
    }
}
