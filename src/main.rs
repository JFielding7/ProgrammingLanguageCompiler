use error::compiler_error::CompilerError::{FileRead, NoInputFiles};
use error::compiler_error::CompilerResult;
use crate::lexer::tokenizer::lex;
use ast::AST;
use std::fs::read_to_string;
use std::path::Path;
use source::source_file::SourceFile;

mod lexer;
mod syntax;
mod error;
mod source;
// mod semantic;
mod ast;

fn compile_program(args: Vec<String>, curr_source_file: &mut Option<SourceFile>) -> CompilerResult {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let file_name = args[1].to_string();
    let path = Path::new(&file_name);
    let source_code = read_to_string(path)
        .map_err(|error| FileRead { file_name: file_name.clone(), error })?;

    let file = SourceFile::new(
        path.to_path_buf(),
        source_code.lines().map(|line| line.to_string()).collect::<Vec<String>>()
    );
    let source_file = curr_source_file.insert(file);

    let source_lines = lex(source_file)?;

    let ast: AST = source_lines.try_into()?;
    
    println!("{:?}", ast);
    
    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();
    let mut curr_source_file: Option<SourceFile> = None;

    if let Err(err) = compile_program(args, &mut curr_source_file) {
        println!("{}", err.format(curr_source_file));
    }
}
