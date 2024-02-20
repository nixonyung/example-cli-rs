use anyhow::Context;
use std::io::{self, Write};

// (ref.) [Why is it discouraged to accept a reference &String, &Vec, or &Box as a function argument?](https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-string-vec-or-box-as-a-function)
pub fn input(prompt_str: &str) -> anyhow::Result<String> {
    print!("{prompt_str}");
    io::stdout()
        .flush()
        .context("unexpected error during stdout.flush")?;

    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .context("unexpected error during stdin.read_line")?;

    Ok(buf.trim().to_owned())
}
