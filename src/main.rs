use std::*;

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("error: no found io option");
        return process::ExitCode::FAILURE;
    }
    process::ExitCode::SUCCESS
}
