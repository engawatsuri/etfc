use std::*;
use std::io::Read;
use std::io::Write;

mod public {
    pub fn to_utf16_external_plain_text() -> process::ExitCode {
        let mut stdin: Vec<u8> = Vec::new();
        let mut stdout: Vec<u8> = Vec::new();

        if let Err(_e) = io::stdin().read_to_end(&mut stdin) {
            eprintln!("error: cannnot read data");
            return process::ExitCode::FAILURE;
        }
        stdout = if cfg!(target_endian = "little") {
            vec![0xFF, 0xFE]
        } else if cfg!(target_endian = "big") {
            vec![0xFE, 0xFF]
        };
        stdout.extend(&stdin);
        if let Err(_e) = io::stdout().write_all(&stdout) && let Err(_e) = io::stdout().flush() {
            eprintln!("error: cannnot write data");
            return process::ExitCode::FAILURE;
        }

        process::ExitCode::SUCCESS
    }
}

pub static FROM: sync::LazyLock<collections::HashMap<&'static str, fn() -> process::ExitCode>> = sync::LazyLock::new(|| {
    let mut map: collections::HashMap<&'static str, fn() -> process::ExitCode> = collections::HashMap::new();
    map.insert("public.utf16-external-plain-text", public::to_utf16_external_plain_text);
    map
});
