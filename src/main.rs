use error::compiler_error::CompilerError::{FileRead, NoInputFiles};
use error::compiler_error::CompilerResult;
use crate::lexer::tokenizer::lex;
use crate::syntax::ast::AST;
use std::fs::read_to_string;
use std::path::Path;
use crate::lexer::error::LexerError;
use source::source_file::SourceFile;
use crate::syntax::error::SyntaxError;

mod lexer;
mod syntax;
mod error;
mod source;

fn compile_program(args: Vec<String>) -> CompilerResult {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let file_name = args[1].to_string();
    let path = Path::new(&file_name);
    let src = read_to_string(path)
        .map_err(|error| FileRead { file_name: file_name.clone(), error })?;

    let f = SourceFile::new(path.to_path_buf(), src.lines().map(|s| s.to_string()).collect::<Vec<String>>());

    let source_lines = lex(src).map_err(|e: LexerError| e.compiler_error(&f))?;

    let ast: AST = source_lines.try_into().map_err(|e: SyntaxError| e.compiler_error(&f))?;
    
    println!("{:?}", ast);
    
    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(err) = compile_program(args) {
        println!("{}", err);
    }
}
