mod public;
use std::*;

static CONVERTER: sync::LazyLock<collections::HashMap<&'static str, collections::HashMap<&'static str, fn() -> process::ExitCode>>> = sync::LazyLock::new(|| {
    let mut map: collections::HashMap<&'static str, collections::HashMap<&'static str, fn() -> process::ExitCode>> = collections::HashMap::new();
    map.insert("public.utf8-plain-text", (*public::utf8_plain_text::FROM).clone());
    map
});

/// analyze argument and run converting function
fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    /*
     * length have to be 2
     * [0]: redirected stdin format
     * [1]: redirected stdout format
     */
    let mut req_args: Vec<String> = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    // divide args into stdin_fmt and stdout_fmt and ops
    let mut op_can_exist = true;
    let mut req_arg_count = 0;
    for arg in &args[1..] {
        if op_can_exist && arg.starts_with("--") {
            if arg.len() < 3 {
                op_can_exist = false;
                continue;
            }
            ops.push(arg[2..].to_string());
        } else if op_can_exist && arg.starts_with("-") {
            if arg.len() < 2 {
                eprintln!("error: miss the option");
                return process::ExitCode::FAILURE;
            }
            for c in arg.chars() {
                let op = match c {
                    'h' => "help",
                    'v' => "version",
                    _ => {
                        eprintln!("error: {}: no found shortened option", c);
                        return process::ExitCode::FAILURE;
                    }
                };
                ops.push(op.to_string());
            }
        } else {
            match req_arg_count {
                0 | 1 => {
                    req_args.push(arg.clone());
                }
                _ => {
                    eprintln!("error: too many argument");
                    return process::ExitCode::FAILURE;
                }
            }
            req_arg_count += 1;
        }
    }
    if req_args.len() < 2 {
        eprintln!("error: no found input or output format");
        return process::ExitCode::FAILURE;
    }

    // option process
    for i in 0..ops.len() {
        match ops[i].as_str() {
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
                eprintln!("error: {}: no found option", ops[i]);
                return process::ExitCode::FAILURE;
            }
        }
    }

    // change short format to origin format
    for req_arg in &mut req_args {
        let origin_fmt = match req_arg.as_str() {
            "public.text" => "public.utf8-plain-text",
            "public.plain-text" => "public.utf8-plain-text",
            _ => continue,
        };
        *req_arg = origin_fmt.to_string();
    }

    // run converting function
    let Some(from) = CONVERTER.get(req_args[0].as_str()) else {
        eprintln!("error: {}: this format isn't supported", req_args[0]);
        return process::ExitCode::FAILURE;
    };
    let Some(converter) = from.get(req_args[1].as_str()) else {
        eprintln!("error: {}: this format isn't supported", req_args[0]);
        return process::ExitCode::FAILURE;
    };
    let result = converter();
    if result == process::ExitCode::SUCCESS {
        eprintln!("converted");
    }
    result
}
