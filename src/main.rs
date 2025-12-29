use crate::error::compiler_error::CompilerError::NoInputFiles;
use crate::error::compiler_error::Result;
use crate::lexer::tokenizer::SourceLines;
use crate::syntax::ast::AST;

mod error;
mod lexer;
mod syntax;

fn compile_program(args: Vec<String>) -> Result<()> {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let tokens = SourceLines::tokenize_file(args[1].to_string())?;

    let ast: AST = tokens.try_into()?;
    
    println!("{:?}", ast);
    
    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    println!("{:?}", compile_program(args));
}
