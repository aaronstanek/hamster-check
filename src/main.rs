use std::env;
use std::process::ExitCode;

mod parse_arguments;
use parse_arguments::parse_arguments;

fn main() -> ExitCode {
    let arguments = match parse_arguments(env::args().collect()) {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{}", x);
            return ExitCode::from(1);
        }
    };
    println!("{:#?}", arguments);
    return ExitCode::from(0);
}
