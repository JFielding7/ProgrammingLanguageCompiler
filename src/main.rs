use error::compiler_error::CompilerError::{FileRead, NoInputFiles};
use error::compiler_error::CompilerResult;
use crate::lexer::tokenizer::TokenizedLines;
use std::fs::read_to_string;
use std::path::Path;
use source::source_file::SourceFile;
use crate::ast::arena_ast::AST;
use crate::syntax::parser::ast_parser::ASTParser;

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
    let file_path = Path::new(&file_name);
    let source_code = read_to_string(file_path)
        .map_err(|error| FileRead { file_name: file_name.clone(), error })?;

    let source_file = curr_source_file.insert(
        SourceFile::new(file_path, source_code)
    );

    let source_lines = TokenizedLines::from_source_file(source_file)?;

    let ast: AST = ASTParser::generate_ast(source_lines)?;
    
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
