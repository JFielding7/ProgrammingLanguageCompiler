use std::fs::read_to_string;
use std::path::Path;
use crate::compiler_error::CompilerError::{NoInputFiles, FileRead};
use crate::compiler_error::CompilerResult;
use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::AST;

mod error_util;
mod lexer;
mod syntax;
mod compiler_error;

fn compile_program(args: Vec<String>) -> CompilerResult<()> {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let file_name = args[1].to_string();
    let path = Path::new(&file_name);
    let src = read_to_string(path)
        .map_err(|error| FileRead { file_name: file_name.clone(), error })?;

    let source_lines = SourceLines::lex(path, src)?;

    let ast: AST = source_lines.try_into()?;
    
    println!("{:?}", ast);
    
    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(err) = compile_program(args) {
        println!("{}", err);
    }
}
