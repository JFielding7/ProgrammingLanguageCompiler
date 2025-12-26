use crate::ast::ast_tree::build_ast;
use crate::error::compiler_error::CompilerError::NoInputFiles;
use crate::error::compiler_error::Result;
use crate::lexer::lexer::TokenizedSource;
use parser::statement::ParsedSource;

mod ast;
mod parser;
mod error;
mod lexer;

fn compile_program(args: Vec<String>) -> Result<()> {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let tokens = TokenizedSource::tokenize_file(args[1].to_string())?;

    let ast = build_ast(tokens)?;

    println!("dk won again");

    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(error) = compile_program(args) {
        println!("{error}");
    }
}
