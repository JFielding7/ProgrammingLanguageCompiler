use crate::compiler_error::CompilerError::NoInputFiles;
use crate::tokenizer::tokenize_file;

mod tokenizer;
// mod line;
mod compiler_error;
// mod ast;

fn compile_program(args: Vec<String>) -> compiler_error::Result<()> {
    const MIN_ARG_COUNT: usize = 2;

    if args.len() < MIN_ARG_COUNT {
        return Err(NoInputFiles)
    }

    let tokens = tokenize_file(args[1].to_string())?;

    for line in tokens.lines {
        println!("{line:?}")
    }

    Ok(())
}

fn main()  {
    let args = std::env::args().collect::<Vec<_>>();

    if let Err(error) =  compile_program(args) {
        eprintln!("{error}");
    }
}
