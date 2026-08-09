use std::*;

fn main() -> process::ExitCode {
    let mut args: Vec<String> = env::args().collect();
    let mut stdin_fmt = String::new();
    let mut stdout_fmt = String::new();
    let mut ops: Vec<String> = Vec::new();

    // divide args into stdin_fmt and stdout_fmt and ops
    let mut op_can_exist = true;
    let mut req_arg_count = 0;
    for i in 0..args.len() {
        if op_can_exist && args[i].starts_with("--") {
            if args[i].len() < 3 {
                op_can_exist = false;
                continue;
            }
            ops.push(args[i][2..].to_string());
        } op_can_exist && else if args[i].starts_with("-") {
            if args[i].len() < 2 {
                eprintln!("error: miss the option");
                return process::ExitCode::FAILURE;
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
            match req_arg_count {
                0 => {
                    stdin_fmt = args[i].clone();
                }
                1 => {
                    stdout_fmt = args[i].clone();
                }
                _ => {
                    eprintln("error: too many argument");
                    return process::ExitCode::FAILURE;
                }
            }
            req_arg_count += 1;
        }
    }
    if req_arg_count < 2 {
        eprintln!("error: no found input or output format");
        return process::ExitCode::FAILURE;
    }

    // option process
    for i in 0..ops.len() {
        match ops[i] {
            "help" => {
                println!("Usage: etfc [OPTION]... STDIN_FORMAT STDOUT_FORMAT 0< INPUT_FILE 1> OUTPUT_FILE");
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
                return process::ExitCode::FAILURE;
            }
        }
    }

    process::ExitCode::SUCCESS
}
