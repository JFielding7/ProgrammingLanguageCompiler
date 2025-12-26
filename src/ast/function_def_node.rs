use std::vec::IntoIter;
use crate::ast::ast_node::ASTNode;
use crate::ast::ast_node::ASTNode::FunctionDefinition;
use crate::ast::parameter_node::ParameterNode;
use crate::error::compiler_error::CompilerError::ExpectTokenNotFound;
use crate::error::compiler_error::Result;
use crate::token::{Token, TokenOpt};
use crate::token::TokenType::{CloseParen, Comma, Identifier, OpenParen};

#[derive(Debug)]
pub struct FunctionDefNode {
    name: String,
    params: Vec<ParameterNode>,
    body: Vec<ASTNode>,
}

impl FunctionDefNode {
    pub fn new(mut tokens_iter: IntoIter<Token>) -> Result<ASTNode> {
        let func_name = tokens_iter.next().assert_type(Identifier)?.token_str;

        tokens_iter.next().assert_type(OpenParen)?;

        let mut params = Vec::new();
        let mut next_token = tokens_iter.next().ok_or_else(|| ExpectTokenNotFound(None, CloseParen))?;

        if matches!(next_token.token_type, Identifier) {
            loop {
                let param_type = next_token.token_str;
                let param_name = tokens_iter.next().assert_type(Identifier)?.token_str;

                params.push(ParameterNode::new(param_name, param_type));

                match tokens_iter.next() {
                    None => return Err(ExpectTokenNotFound(None, CloseParen)),
                    Some(token) => {
                        if matches!(token.token_type, CloseParen) {
                            break;
                        }
                        if !matches!(token.token_type, Comma) {
                            return Err(ExpectTokenNotFound(Some(token), Comma));
                        }
                    }
                }

                next_token = tokens_iter.next().assert_type(Identifier)?;
            }
        }
        else if !matches!(next_token.token_type, CloseParen) {
            return Err(ExpectTokenNotFound(Some(next_token), CloseParen))
        }

        Ok(FunctionDefinition(FunctionDefNode {
            name: func_name.clone(),
            params,
            body: vec![],
        }))
    }
}
