use crate::lexer::token::{Token, TokenType};
use crate::lexer::token::TokenType::*;
use crate::syntax::ast::ast_node::ASTNode;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::ast::unary_operator_node::{UnaryOperatorNode, UnaryOperatorType};
use crate::syntax::error::SyntaxError::InvalidExpression;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::expression::OperatorPrecedence::Prefix;
use crate::syntax::parser::statement::Statement;
use crate::syntax::parser::token_stream::TokenStream;

#[repr(u8)]
#[derive(Copy, Clone)]
enum OperatorPrecedence {
    Comma = 0,
    Assign,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    Relational,
    BitShift,
    Add,
    Mul,
    Prefix,
    Postfix,
}

impl OperatorPrecedence {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn left_assoc(self) -> (u8, u8) {
        (self.as_u8(), self.as_u8() + 1)
    }

    fn right_assoc(self) -> (u8, u8) {
        (self.as_u8(), self.as_u8())
    }
}

fn operators_with_lhs_precedence(op: &Token) -> Option<(u8, u8)> {
    use OperatorPrecedence::*;

    Some(match op.token_type {
        TokenType::Comma => Comma.left_assoc(),

        Equals
        | PlusEquals
        | MinusEquals
        | StarEquals
        | SlashEquals
        | PercentEquals
        | DoubleLeftArrowEquals
        | DoubleRightArrowEquals
        | AmpersandEquals
        | CaretEquals
        | PipeEquals
        => Assign.right_assoc(),

        DoublePipe => LogicalOr.left_assoc(),

        DoubleAmpersand => LogicalAnd.left_assoc(),

        Pipe => BitOr.left_assoc(),

        Caret => BitXor.left_assoc(),

        Ampersand => BitAnd.left_assoc(),

        DoubleEquals
        | ExclamationEquals
        => Equality.left_assoc(),

        Less
        | LessEquals
        | Greater
        | GreaterEquals
        => Relational.left_assoc(),

        DoubleLeftArrow
        | DoubleRightArrow
        => BitShift.left_assoc(),

        Plus
        | Minus
        => Add.left_assoc(),

        Star
        | Slash
        | Percent
        => Mul.left_assoc(),

        PlusPlus
        | MinusMinus
        | OpenParen
        | OpenBracket
        | Dot
        => Postfix.left_assoc(),

        _ => return None,
    })
}


fn binary_operator_type(op: &Token) -> Option<BinaryOperatorType> {
    use BinaryOperatorType::*;

    Some(match op.token_type {
        Equals => Assign,
        PlusEquals => AddAssign,
        MinusEquals => SubAssign,
        StarEquals => MulAssign,
        SlashEquals => DivAssign,
        PercentEquals => ModAssign,
        DoubleLeftArrowEquals => LeftShiftAssign,
        DoubleRightArrowEquals => RightShiftAssign,
        AmpersandEquals => AndAssign,
        CaretEquals => XorAssign,
        PipeEquals => OrAssign,

        Plus => Add,
        Minus => Sub,
        Star => Mul,
        Slash => Div,
        Percent => Mod,

        Ampersand => BitAnd,
        Pipe => BitOr,
        Caret => BitXor,

        DoubleLeftArrow => LeftShift,
        DoubleRightArrow => RightShift,

        DoubleEquals => Equal,
        ExclamationEquals => NotEquals,
        Less => LessThan,
        LessEquals => LessOrEqual,
        Greater => GreaterThan,
        GreaterEquals => GreaterOrEqual,

        DoubleAmpersand => LogicalAnd,
        DoublePipe => LogicalOr,

        Comma => CommaOperator,

        _ => return None,
    })
}

fn prefix_unary_operator_type(op: &Token) -> Option<UnaryOperatorType> {
    use UnaryOperatorType::*;

   Some(match op.token_type {
        Minus => Neg,
        Exclamation => Not,
        Tilde => BitNot,
        PlusPlus => PreInc,
        MinusMinus => PreDec,
        Dot => Deref,
        _ => return None,
    })
}

fn postfix_unary_operator_type(op: &Token) -> Option<UnaryOperatorType> {
    use UnaryOperatorType::*;

    Some(match op.token_type {
        PlusPlus => PostInc,
        MinusMinus => PostDec,
        _ => return None,
    })
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

fn nud_hook(token_stream: &mut TokenStream) -> SyntaxResult<ASTNode> {

    match token_stream.next() {
        None => Err(InvalidExpression(token_stream.prev_location())),

        Some(token) => {
            if let Some(unary_op_type) = prefix_unary_operator_type(token) {
                Ok(UnaryOperatorNode::new(
                    unary_op_type,
                    parse_expression_rec(token_stream, Prefix.as_u8())?
                ).into())

            } else if *token == OpenParen {
                let paren_expr = parse_expression_rec(
                    token_stream, 0
                );
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

    let mut left_node = nud_hook(token_stream)?;

    while let Some(&token) = token_stream.peek() {

        if *token == CloseParen {
            return Ok(left_node);
        }

        if let Some((left_precedence, right_precedence)) = operators_with_lhs_precedence(token) {
            if left_precedence < curr_precedence {
                return Ok(left_node)
            }

            token_stream.next();

            // TODO: led hook for function call and indexing
            left_node = if let Some(op_type) = binary_operator_type(token) {
                let right_node = parse_expression_rec(token_stream, right_precedence)?;
                BinaryOperatorNode::new(op_type, left_node, right_node).into()

            } else if let Some(op_type) = postfix_unary_operator_type(token) {
                UnaryOperatorNode::new(op_type, left_node).into()

            } else {
                unreachable!("Led hook failed for {token}");
            }
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
