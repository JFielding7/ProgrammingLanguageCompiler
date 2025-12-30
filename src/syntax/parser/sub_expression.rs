use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::{Equals, CloseParen, Identifier, IntLiteral, Minus, OpenParen, Plus, StringLiteral, Star, Slash};
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::error::SyntaxError::InvalidExpression;
use crate::syntax::error::SyntaxResult;

const OPERATOR_GROUPS_COUNT: usize = 2;
const OPERATORS: [&[TokenType]; OPERATOR_GROUPS_COUNT] = [
    &[Equals],
    &[Plus, Minus]
];

pub struct SubExpressionParser<'a> {
    tokens: &'a [Token],
    paren_matches: &'a Vec<usize>,
    start_op_group: usize,
}

impl <'a> SubExpressionParser<'a> {

    pub fn new(tokens: &'a [Token], paren_matches: &'a Vec<usize>) -> Self {
        Self {
            tokens,
            paren_matches,
            start_op_group: 0
        }
    }

    fn sub_expression(&'a self, start: usize, end: usize, start_op_group: usize) -> Self {
        Self {
            tokens: &self.tokens[start..end],
            paren_matches: self.paren_matches,
            start_op_group
        }
    }

    fn remove_redundant_parens(&mut self) {
        let max_index = self.len() - 1;
        let mut start = 0;

        while self.tokens[start] == OpenParen && self.tokens[max_index - start] == CloseParen {
            start += 1;
        }

        if start != 0 {
            self.tokens = &self.tokens[start..max_index - start + 1];
            self.start_op_group = 0;
        }
    }

    pub fn parse(&mut self) -> SyntaxResult<ASTNode> {
        self.remove_redundant_parens();

        let tokens_len = self.tokens.len();
        // TODO: 0 length

        if tokens_len == 1 {
            return parse_value(&self.tokens[0]);
        }

        while self.start_op_group < OPERATOR_GROUPS_COUNT {
            let mut i = self.tokens.len() as isize - 1;

            while i >= 0 {
                let index = i  as usize;

                if in_operator_group(self.start_op_group, &self.tokens[index]) {
                    let op_token = &self.tokens[index];

                    let left_node = self
                        .sub_expression(0, index, self.start_op_group)
                        .parse()?;

                    let right_node = self
                        .sub_expression(index + 1, self.len(), self.start_op_group)
                        .parse()?;

                    return Ok(BinaryOperatorNode::new(op_token.into(), left_node, right_node).into());
                    
                } else if self.tokens[index] == CloseParen {
                    i = self.paren_matches[index] as isize;
                } else {
                    i -= 1;
                }
            }

            self.start_op_group += 1;
        }

        Err(InvalidExpression(self.tokens[0].error_location.clone()))
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }
}

fn parse_value(token: &Token) -> SyntaxResult<ASTNode> {
    let token_string = token.token_str.clone();

    match token.token_type {
        IntLiteral    => Ok(ASTNode::IntLiteral(token_string)),
        StringLiteral => Ok(ASTNode::StringLiteral(token_string)),
        Identifier    => Ok(ASTNode::Identifier(token_string)),
        _ => Err(InvalidExpression(token.error_location.clone()))
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
