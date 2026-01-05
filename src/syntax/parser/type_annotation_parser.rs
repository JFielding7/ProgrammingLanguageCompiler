use crate::lexer::token::Token;
use crate::lexer::token::TokenType::{Colon, Comma, Greater, Less};
use crate::syntax::ast::ast_node::TypeAnnotation;
use crate::syntax::error::SyntaxResult;
use crate::syntax::parser::token_stream::TokenStream;

fn parse_inner_types(token_stream: &mut TokenStream) -> SyntaxResult<Vec<TypeAnnotation>> {

    let mut inner_types = vec![parse_type_annotation_rec(token_stream)?];

    while token_stream.next_matches(Comma) {
        token_stream.next();

        inner_types.push(parse_type_annotation_rec(token_stream)?);
    }

    Ok(inner_types)
}

fn parse_type_annotation_rec(token_stream: &mut TokenStream) -> SyntaxResult<TypeAnnotation> {

    let type_name = token_stream.expect_next_identifier()?;

    if token_stream.next_matches(Less) {
        token_stream.next();
        let inner_types = parse_inner_types(token_stream)?;
        token_stream.expect_next_token(Greater)?;

        Ok(TypeAnnotation::with_params(type_name, inner_types))
    } else {
        Ok(TypeAnnotation::new(type_name))
    }
}

pub fn parse_type_annotation(token_stream: &mut TokenStream, terminal_token: &Token) -> SyntaxResult<Option<TypeAnnotation>> {
    Ok(if *terminal_token == Colon {
        token_stream.next();
        Some(parse_type_annotation_rec(token_stream)?)
    } else {
        None
    })
}
