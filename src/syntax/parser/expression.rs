use crate::lexer::token::Token;
use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::binary_operator_node::BinaryOperatorType::*;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::ast::unary_operator_node::UnaryOperatorType::Neg;
use crate::syntax::ast::unary_operator_node::{UnaryOperatorNode, UnaryOperatorType};
use crate::syntax::error::SyntaxError::InvalidExpression;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::token_stream::TokenStream;

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

fn binary_operator_precedence(op: &Token) -> Option<(u8, u8)> {
    match op.token_type {
        Equals => Some((1, 0)),
        Plus | Minus => Some((3, 4)),
        Star | Slash | Percent => Some((5, 6)),
        _ => None,
    }
}

fn unary_operator_type(op: &Token) -> Option<UnaryOperatorType> {
    match op.token_type {
        Minus => Some(Neg),
        _ => None,
    }
}

fn parse_token(token: &Token) -> SyntaxResult<ASTNode> {
    let token_string = token.to_string();

    match token.token_type {
        IntLiteral    => Ok(ASTNode::IntLiteral(token_string)),
        StringLiteral => Ok(ASTNode::StringLiteral(token_string)),
        Identifier    => Ok(ASTNode::Identifier(token_string)),
        _ => Err(InvalidExpression(token.location.clone()))
    }
}

fn nud(token_stream: &mut TokenStream) -> SyntaxResult<ASTNode> {

    match token_stream.next() {
        None => Err(InvalidExpression(token_stream.prev_location())),

        Some(token) => {
            // println!("{}", token);
            if let Some(unary_op_type) = unary_operator_type(token) {
                Ok(UnaryOperatorNode::new(unary_op_type, nud(token_stream)?).into())
            } else if *token == OpenParen {
                let paren_expr = parse_expression_rec(token_stream, 0);
                token_stream.next();
                paren_expr
            } else {
                parse_token(token)
            }
        }
    }
}

fn parse_expression_rec(token_stream: &mut TokenStream, curr_precedence: u8) -> SyntaxResult<ASTNode> {

    if token_stream.empty() {
        return Err(InvalidExpression(token_stream.prev_location()))
    }

    let mut left_node = nud(token_stream)?;

    while let Some(&token) = token_stream.peek() {

        if *token == CloseParen {
            return Ok(left_node)
        }

        if let Some((left_precedence, right_precedence)) = binary_operator_precedence(token) {

            if left_precedence < curr_precedence {
                return Ok(left_node)
            }

            token_stream.next();

            let right_node = parse_expression_rec(token_stream, right_precedence)?;
            left_node = BinaryOperatorNode::new(token.into(), left_node, right_node).into();

        } else {
            return Err(InvalidExpression(token.location.clone()));
        }
    }

    Ok(left_node)
}

pub fn parse_expression(statement: &Statement, start: usize) -> SyntaxResult<ASTNode> {
    let mut token_stream = TokenStream::from_statement_suffix(statement, start);

    parse_expression_rec(&mut token_stream, 0)
}
