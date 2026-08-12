use std::*;
use std::io::Read;
use std::io::Write;

mod public {
    pub fn to_utf16_plain_text() -> process::ExitCode {
        let mut stdin: Vec<u8> = Vec::new();
        let mut stdout: Vec<u8> = Vec::new();

        if let Err(_e) = io::stdin().read_to_end(&mut stdin) {
            eprintln!("error: cannnot read data");
            return process::ExitCode::FAILURE;
        }
        let endian = match stdin.get(..2) {
            Some(bom) => {
                match bom {
                    vec![0xFF, 0xFE] => {
                        stdin.drain(..2);
                        "little"
                    }
                    vec![0xFE, 0xFF] => {
                        stdin.drain(..2);
                        "big"
                    }
                    _ => "big",
                }
            }
            None => {
                eprintln("error: missing format");
                return process::ExitCode::FAILURE;
            }
        };
        let target_endian = if cfg!(target_endian = "little") {
            "little"
        } else if cfg!(target_endian = "big") {
            "big"
        };
        if endian != target_endian {
            for i in 0..stdin.len() {
                if i % 2 == 1 {
                    stdout.extend(&stdin[(i-1)..=i]);
                }
            }
        } else {
            stdout = stdin.clone();
        }
        if let Err(_e) = io::stdout().write_all(&stdout) && let Err(_e) = io::stdout().flush() {
            eprintln!("error: cannnot write data");
            return process::ExitCode::FAILURE;
        }

        process::ExitCode::SUCCESS
    }
}

pub static FROM: sync::LazyLock<collections::HashMap<&'static str, fn() -> process::ExitCode>> = sync::LazyLock::new(|| {
    let mut map: collections::HashMap<&'static str, fn() -> process::ExitCode> = collections::HashMap::new();
    map.insert("public.utf16-plain-text", public::to_utf16_plain_text);
    map
});
