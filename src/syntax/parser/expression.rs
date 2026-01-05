use crate::error::spanned_error::WithSpan;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::{Token, TokenType};
use crate::syntax::ast::access_node::{AccessNode, Member};
use crate::syntax::ast::ast_node::{ASTNode, ASTNodeType};
use crate::syntax::ast::ast_node::ASTNodeSpan;
use crate::syntax::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::syntax::ast::function_call_node::FunctionCallNode;
use crate::syntax::ast::index_node::IndexNode;
use crate::syntax::ast::unary_operator_node::{UnaryOperatorNode, UnaryOperatorType};
use crate::syntax::error::SyntaxErrorType::InvalidExpression;
use crate::syntax::error::{SyntaxErrorType, SyntaxResult};
use crate::syntax::parser::expression::OperatorPrecedence::Prefix;
use crate::syntax::parser::token_stream::TokenStream;
use crate::syntax::parser::type_annotation_parser::parse_type_annotation;

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

fn is_terminal(token: &Token) -> bool {
    matches!(token.token_type, CloseParen | CloseBracket | Colon)
}

fn close_token(open_token: &Token) -> TokenType {
    use TokenType::*;
    
    match open_token.token_type {
        OpenParen => CloseParen,
        OpenBracket => CloseBracket,
        _ => unreachable!("Invalid group opening token: {open_token}"),
    }
}

fn assert_group_closed(token_stream: &mut TokenStream, open_token: &Token) -> SyntaxResult<()> {
    if token_stream.next_matches(close_token(open_token)) {
        token_stream.next();
        Ok(())
    } else {
        Err(SyntaxErrorType::unmatched_paren(open_token.token_type.clone()).at(open_token.span.clone()))
    }
}

fn parse_required_grouped_expression(token_stream: &mut TokenStream, open_token: &Token) -> SyntaxResult<ASTNode> {
    let group = parse_expression_rec(token_stream, 0)?;
    
    assert_group_closed(token_stream, open_token)?;
    Ok(group)
}

fn parse_optional_grouped_expression(token_stream: &mut TokenStream, open_token: &Token) -> SyntaxResult<Option<ASTNode>> {
    let group = if token_stream.next_matches(CloseParen) {
        None
    } else {
        Some(parse_expression_rec(token_stream, 0)?)
    };

    assert_group_closed(token_stream, open_token)?;
    Ok(group)
}

fn parse_accessed_member(token_stream: &mut TokenStream) -> SyntaxResult<Member> {
    let member_name = token_stream.expect_next_token(Identifier)?;
    let member_name_string = member_name.to_string();

    if token_stream.next_matches(OpenParen) {
        token_stream.next();

        let member = if token_stream.next_matches(CloseParen) {
            Ok(Member::method_no_args(member_name_string))
        } else {
            let args = parse_expression_rec(token_stream, 0)?;
            Ok(Member::method_with_args(member_name_string, args))
        };
        token_stream.next();
        member

    } else {
        Ok(Member::field(member_name_string))
    }
}

fn parse_token(token: &Token) -> SyntaxResult<ASTNode> {
    use ASTNodeType::*;

    let token_string = token.to_string();
    let token_span = token.span.clone();

    Ok(match token.token_type {
        TokenType::IntLiteral    => ASTNode::new(IntLiteral(token_string), token_span),
        TokenType::StringLiteral => ASTNode::new(StringLiteral(token_string), token_span),
        TokenType::Identifier    => ASTNode::new(Variable(token_string), token_span),
        _ => return Err(InvalidExpression.at(token.span.clone()))
    })
}

fn nud_hook(token_stream: &mut TokenStream) -> SyntaxResult<ASTNode> {

    match token_stream.next() {
        None => Err(InvalidExpression.at(token_stream.prev_span())),

        Some(token) => {
            if let Some(unary_op_type) = prefix_unary_operator_type(token) {
                Ok(UnaryOperatorNode::new(
                    unary_op_type,
                    parse_expression_rec(token_stream, Prefix.as_u8())?
                ).at(token.span.clone()))

            } else if *token == OpenParen {
                parse_required_grouped_expression(token_stream, token)

            } else {
                parse_token(token)
            }
        }
    }
}

fn parse_expression_rec(token_stream: &mut TokenStream, curr_precedence: u8) -> SyntaxResult<ASTNode> {

    if token_stream.empty() {
        return Err(InvalidExpression.at(token_stream.prev_span()))
    }

    let mut left_node = nud_hook(token_stream)?;

    while let Some(&token) = token_stream.peek() {

        if is_terminal(token) {
            return Ok(left_node.annotate_type(
                parse_type_annotation(token_stream, token)?)
            );
        }

        if let Some((left_precedence, right_precedence)) = operators_with_lhs_precedence(token) {
            if left_precedence < curr_precedence {
                return Ok(left_node)
            }

             token_stream.next();
            let token_span = token.span.clone();

            left_node = if let Some(op_type) = binary_operator_type(token) {
                let right_node = parse_expression_rec(token_stream, right_precedence)?;
                BinaryOperatorNode::new(op_type, left_node, right_node).at(token_span)

            } else if let Some(op_type) = postfix_unary_operator_type(token) {
                UnaryOperatorNode::new(op_type, left_node).at(token_span)

            } else if *token == OpenBracket {
                let args = parse_required_grouped_expression(token_stream, token)?;
                IndexNode::new(left_node, args).at(token_span)

            } else if *token == OpenParen {
                let args = parse_optional_grouped_expression(token_stream, token)?;
                FunctionCallNode::new(left_node, args).at(token_span)

            } else if *token == Dot {
                let member = parse_accessed_member(token_stream)?;
                AccessNode::new(left_node, member).at(token_span)

            } else {
                unreachable!("Led hook not implemented for {token}");
            }
        } else {
            return Err(InvalidExpression.at(token.span.clone()));
        }
    }

    Ok(left_node)
}

pub fn parse_expression(mut token_stream: TokenStream) -> SyntaxResult<ASTNode> {
    parse_expression_rec(&mut token_stream, 0)
}
