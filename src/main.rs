use std::*;

fn main() -> process::ExitCode {
    let mut args: Vec<String> = env::args().collect();
    let mut req_args: Vec<String> = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    for i in 0..args.len() {
        if args[i].starts_with("--") {
            ops.push(args[i][2..].to_string());
        } else if args[i].starts_with("-") {
            if args[i].len() == 1 {
                continue;
            }
            for c in args[i][1..] {
                let op = match c {
                    "h" => "help",
                    "v" => "version",
                    _ => {
                        eprintln("error: {}: no found shortened option", c);
                        return process::ExitCode::FAILURE;
                    }
                };
                ops.push(op.to_string());
            }
        } else {
            req_args.push(args[i].clone());
        }
    }
    for i in 0..ops.len() {
        match ops[i] {
            "help" => {
                println!("Usage: etfc [OPTION]... <stdin_format> <stdout_format>");
                println!("");
                println!("Converter the FILE");
                println!("");
                println!("Mandatory arguments to long options are mandatory for short options too.");
                println!("  -h, --help       display this help and exit");
                println!("  -v, --version    display this version and exit");
                return process::ExitCode::SUCCESS;
            }
            "version" => {
                println!("version: 0.1.0");
                return process::ExitCode::SUCCESS;
            }
            _ => {
                eprintln("error: {}: no found option", ops[i]);
                return process::ExitCode::SUCCESS;
            }
        }
    }
    if req_args.len() < 2 {
        eprintln!("error: no found input or output format");
        return process::ExitCode::FAILURE;
    }
    process::ExitCode::SUCCESS
}
