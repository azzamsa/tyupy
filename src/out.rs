use std::io::{self, Write};

pub fn stdout(input: &str) {
    writeln!(io::stdout(), "{}", input).ok();
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{}", input).ok();
}
