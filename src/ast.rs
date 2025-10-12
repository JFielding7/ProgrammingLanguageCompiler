use std::vec::IntoIter;
use crate::ast::ASTNode::FunctionDefinition;
use crate::compiler_error::CompilerError::ExpectTokenNotFound;
use crate::lexer::TokenType::{CloseParen, Comma, Identifier, OpenParen};
use crate::lexer::{CheckTokenType, Token};
use crate::statement::SourceFileStatements;

#[derive(Debug)]
enum ASTNode {
    IntLiteral(String),
    StringLiteral(String),

    Variable(VariableNode),

    BinaryOperator(BinaryOperatorNode),

    FunctionDefinition(FunctionDefNode),

    FunctionCall(FunctionCallNode),

    SourceFile {
        name: String,
        functions: Vec<FunctionDefNode>
    }
}

#[derive(Debug)]
struct VariableNode {
    name: String,
    data_type: String,
}

#[derive(Debug)]
struct BinaryOperatorNode {
    op: BinaryOperatorNodeType,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
}

#[derive(Debug)]
enum BinaryOperatorNodeType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct FunctionDefNode {
    name: String,
    params: Vec<VariableNode>,
    body: Vec<ASTNode>,
}

#[derive(Debug)]
struct FunctionCallNode {
    function: Box<ASTNode>,
    params: Vec<ASTNode>,
}

impl VariableNode {
    fn new(name: String, data_type: String) -> Self {
        Self { name, data_type }
    }
}

fn function_def_node(mut tokens_iter: IntoIter<Token>) -> crate::compiler_error::Result<ASTNode> {
    let func_name = tokens_iter.next().check_type(Identifier)?.token_str;

    tokens_iter.next().check_type(OpenParen)?;

    let mut params = Vec::new();
    let mut next_token = tokens_iter.next().ok_or_else(|| ExpectTokenNotFound(None, CloseParen))?;

    if matches!(next_token.token_type, Identifier) {
        loop {
            let param_type = next_token.token_str;
            let param_name = tokens_iter.next().check_type(Identifier)?.token_str;

            params.push(VariableNode::new(param_name.clone(), param_type.clone()));

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

            next_token = tokens_iter.next().check_type(Identifier)?;
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

fn get_ast_node(statement: Vec<Token>) -> crate::compiler_error::Result<ASTNode> {
    let mut tokens = statement.into_iter();

    match tokens.next().unwrap().token_type {
        _ => {
            Ok(function_def_node(tokens)?)
        }
    }
}

pub fn build_ast(statements: SourceFileStatements) {
    for statement in statements.statements {
        println!("{:?}", get_ast_node(statement));
    }
}
