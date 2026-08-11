use std::*;

pub fn to_new__daringfireball__markdown() -> process::ExitCode {
    let mut stdin: Vec<u8> = Vec::new();
    let mut stdout: Vec<u8> = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut stdin) {
        eprintln("error: cannnot read data");
        return process::ExitCode::FAILURE;
    }
    for byte in stdin {
        match byte {
            0x0a => {
                stdout.push(0x20);
                stdout.push(0x20);
            }
            0x21 | 0x23 | 0x28 | 0x29 | 0x2a | 0x2b | 0x2d | 0x2e | 0x3e | 0x5b | 0x5c | 0x5d | 0x5f | 0x60 | 0x7e => {
                stdout.push(0x5c);
            }
        }
        stdout.push(byte);
    }
    if let Err(e) = io::stdout().write_all(&stdout) || let Err(e) = io::stdout().flush() {
        eprintln("error: cannnot write data");
        return process::ExitCode::FAILURE;
    }

    process::ExitCode::SUCCESS
}

static FROM_UTF8_PLAIN_TEXT: sync::LazyLock<collections::HashMap<&'static str, fn() -> process::ExitCode>> = sync::LazyLock::new(|| {
    let mut map: collections::HashMap<&'static str, fn() -> process::ExitCode> = collections::HashMap::new();
    map.insert("new.daringfireball.markdown", to_new__daringfireball__markdown);
    map
});
