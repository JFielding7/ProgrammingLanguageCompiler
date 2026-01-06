use crate::error::spanned_error::WithSpan;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::{Token, TokenType};
use crate::ast::access_node::{AccessNode, Member};
use crate::ast::ast_arena::{ASTArena, ASTNodeId};
use crate::ast::ast_node::{ASTNode, ASTNodeType};
use crate::ast::binary_operator_node::{BinaryOperatorNode, BinaryOperatorType};
use crate::ast::function_call_node::FunctionCallNode;
use crate::ast::index_node::IndexNode;
use crate::ast::unary_operator_node::{UnaryOperatorNode, UnaryOperatorType};
use crate::syntax::error::SyntaxErrorType::InvalidExpression;
use crate::syntax::error::{SyntaxErrorType, SyntaxResult};
use crate::syntax::parser::expression::OperatorPrecedence::Prefix;
use crate::syntax::parser::token_stream::TokenStream;
use crate::syntax::parser::type_annotation::parse_type_annotation;

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

pub struct ExpressionParser<'a> {
    token_stream: &'a mut TokenStream<'a>,
    ast_arena: &'a mut ASTArena,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(token_stream: &'a mut TokenStream<'a>, ast_arena: &'a mut ASTArena) -> Self {
        Self {
            token_stream,
            ast_arena,
        }
    }

    fn parse_token(&mut self, token: &Token) -> SyntaxResult<ASTNodeId> {
        use ASTNodeType::*;

        let token_string = token.to_string();
        let token_span = token.span.clone();

        let node = match token.token_type {
            TokenType::IntLiteral    => ASTNode::new(IntLiteral(token_string), token_span),
            TokenType::StringLiteral => ASTNode::new(StringLiteral(token_string), token_span),
            TokenType::Identifier    => ASTNode::new(Variable(token_string), token_span),
            _ => return Err(InvalidExpression.at(token.span.clone()))
        };

        Ok(self.ast_arena.add_node(node))
    }

    fn assert_group_closed(&mut self, open_token: &Token) -> SyntaxResult<()> {
        if self.token_stream.peek_matches(close_token(open_token)) {
            self.token_stream.next();
            Ok(())
        } else {
            Err(SyntaxErrorType::unmatched_paren(open_token.token_type.clone()).at(open_token.span.clone()))
        }
    }

    fn parse_required_grouped_expression(&mut self, open_token: &Token) -> SyntaxResult<ASTNodeId> {
        let group = self.parse_expression_rec(0)?;

        self.assert_group_closed(open_token)?;
        Ok(group)
    }

    fn parse_optional_grouped_expression(&mut self, open_token: &Token) -> SyntaxResult<Option<ASTNodeId>> {
        let group = if self.token_stream.peek_matches(CloseParen) {
            None
        } else {
            Some(self.parse_expression_rec(0)?)
        };

        self.assert_group_closed(open_token)?;
        Ok(group)
    }

    fn parse_accessed_member(&mut self) -> SyntaxResult<Member> {
        let member_name = self.token_stream.expect_next_token(Identifier)?;
        let member_name_string = member_name.to_string();

        if self.token_stream.peek_matches(OpenParen) {
            self.token_stream.next();

            let member = if self.token_stream.peek_matches(CloseParen) {
                Ok(Member::method_no_args(member_name_string))
            } else {
                let args = self.parse_expression_rec(0)?;
                Ok(Member::method_with_args(member_name_string, args))
            };
            self.token_stream.next();
            member

        } else {
            Ok(Member::field(member_name_string))
        }
    }

    fn nud_hook(&mut self) -> SyntaxResult<ASTNodeId> {

        match self.token_stream.next() {
            None => Err(InvalidExpression.at(self.token_stream.prev_span())),

            Some(token) => {
                if let Some(unary_op_type) = prefix_unary_operator_type(token) {
                    let unary_node = UnaryOperatorNode::new(
                        unary_op_type,
                        self.parse_expression_rec(Prefix.as_u8())?
                    );
                    Ok(self.ast_arena.add_with_span(unary_node, token.span.clone()))

                } else if *token == OpenParen {
                    self.parse_required_grouped_expression(token)

                } else {
                    self.parse_token(token)
                }
            }
        }
    }

    fn parse_expression_rec(&mut self, curr_precedence: u8) -> SyntaxResult<ASTNodeId> {

        if self.token_stream.empty() {
            return Err(InvalidExpression.at(self.token_stream.prev_span()))
        }

        let mut left_node = self.nud_hook()?;

        while let Some(&token) = self.token_stream.peek() {

            if is_terminal(token) {
                let type_annotation = if *token == Colon {
                    self.token_stream.next();
                    Some(parse_type_annotation(&mut self.token_stream)?)
                } else {
                    None
                };

                return Ok(self.ast_arena.annotate(left_node, type_annotation));
            }

            if let Some((left_precedence, right_precedence)) = operators_with_lhs_precedence(token) {
                if left_precedence < curr_precedence {
                    return Ok(left_node)
                }

                self.token_stream.next();
                let token_span = token.span.clone();

                left_node = if let Some(op_type) = binary_operator_type(token) {
                    let right_node = self.parse_expression_rec(right_precedence)?;
                    self.ast_arena.add_with_span(BinaryOperatorNode::new(op_type, left_node, right_node), token_span)

                } else if let Some(op_type) = postfix_unary_operator_type(token) {
                    self.ast_arena.add_with_span(UnaryOperatorNode::new(op_type, left_node), token_span)

                } else if *token == OpenBracket {
                    let args = self.parse_required_grouped_expression(token)?;
                    self.ast_arena.add_with_span(IndexNode::new(left_node, args), token_span)

                } else if *token == OpenParen {
                    let args = self.parse_optional_grouped_expression(token)?;
                    self.ast_arena.add_with_span(FunctionCallNode::new(left_node, args), token_span)

                } else if *token == Dot {
                    let member = self.parse_accessed_member()?;
                    self.ast_arena.add_with_span(AccessNode::new(left_node, member), token_span)

                } else {
                    unreachable!("Led hook not implemented for {token}");
                };
            } else {
                return Err(InvalidExpression.at(token.span.clone()));
            }
        }

        Ok(left_node)
    }

    pub fn parse(token_stream: &'a mut TokenStream<'a>, ast_arena: &'a mut ASTArena) -> SyntaxResult<ASTNodeId> {
        let mut parser = ExpressionParser::new(token_stream, ast_arena);
        parser.parse_expression_rec(0)
    }
}


