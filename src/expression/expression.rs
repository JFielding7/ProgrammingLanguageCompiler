use crate::ast::ast_node::ASTNode;
use crate::error::compiler_error::Result;
use crate::error::compiler_error::CompilerError::{InvalidExpression, UnmatchedParen};
use crate::token::TokenType::*;
use crate::token::{Token, TokenType};


const OPERATOR_GROUPS_COUNT: usize = 2;
const OPERATORS: [&[TokenType]; OPERATOR_GROUPS_COUNT] = [&[Assign], &[Plus, Minus]];

pub struct ExpressionParser {
    tokens: Vec<Token>,
    paren_matches: Vec<usize>,
}

struct SubExpressionParser<'a> {
    tokens: &'a [Token],
    paren_matches: &'a Vec<usize>,
    start_op_group: usize,
}

impl ExpressionParser {
    fn try_new(
        tokens: Vec<Token>,
    ) -> Result<Self> {
        let paren_matches = Self::match_parens(&tokens)?;

        Ok(Self {
            tokens,
            paren_matches,
        })
    }

    pub fn parse(tokens: Vec<Token>) -> Result<ASTNode> {
        let parser = Self::try_new(tokens)?;

        SubExpressionParser::new(
            &parser.tokens,
            &parser.paren_matches,
        ).parse()
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

impl <'a> SubExpressionParser<'a> {

    fn new(tokens: &'a [Token], paren_matches: &'a Vec<usize>) -> Self {
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

    fn parse_value(token: &Token) -> Result<ASTNode> {
        let token_string = token.token_str.clone();

        match token.token_type {
            IntLiteral => Ok(ASTNode::int_iteral(token_string)),
            StringLiteral => Ok(ASTNode::string_iteral(token_string)),
            Identifier => Ok(ASTNode::identifier(token_string)),
            _ => Err(InvalidExpression(token.error_info))
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

    fn parse(&mut self) -> Result<ASTNode> {
        self.remove_redundant_parens();

        let tokens_len = self.tokens.len();
        // TODO: 0 length

        if tokens_len == 1 {
            return Self::parse_value(&self.tokens[0]);
        }

        while self.start_op_group < OPERATOR_GROUPS_COUNT {
            let mut i = self.tokens.len() as isize - 1;

            while i >= 0 {
                let index = i  as usize;

                if Self::in_operator_group(self.start_op_group, &self.tokens[index]) {
                    let op_token = self.tokens[index].clone();

                    let left_node = self
                        .sub_expression(0, index, self.start_op_group)
                        .parse()?;

                    let right_node = self
                        .sub_expression(index + 1, self.len(), self.start_op_group)
                        .parse()?;

                    return Ok(ASTNode::binary_operator(
                        op_token,
                        Box::new(left_node),
                        Box::new(right_node),
                    ))
                } else if self.tokens[index] == CloseParen {
                    i = self.paren_matches[index] as isize;
                } else {
                    i -= 1;
                }
            }

            self.start_op_group += 1;
        }

        Err(InvalidExpression(self.tokens[0].error_info))
    }
    
    fn len(&self) -> usize {
        self.tokens.len()
    }
}
