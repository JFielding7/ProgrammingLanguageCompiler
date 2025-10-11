use crate::compiler_error::CompilerError::NoInputFiles;
use crate::lexer::tokenize_file;
use crate::statement::get_statements;

mod lexer;
mod compiler_error;
// mod ast;
mod statement;

fn compile_program(args: Vec<String>) -> compiler_error::Result<()> {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let tokens = tokenize_file(args[1].to_string())?;

    let statements = get_statements(tokens)?;

    for statement in &statements.statements {
        println!("{statement:?}\n");
    }

    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(error) =  compile_program(args) {
        eprintln!("{error}");
    }
}
