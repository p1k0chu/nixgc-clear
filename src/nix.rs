use crate::error::{Error, Result};
use crate::fs::can_delete;
use std::{
    io::{self, BufRead},
    path::Path,
    process::{Command, Stdio},
};

pub fn get_gc_roots() -> Result<Vec<String>> {
    let child = Command::new("nix-store")
        .args(["--gc", "--print-roots"])
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(Error::ChildFailed(output.status.code()));
    }

    Ok(output
        .stdout
        .lines()
        .take_while(|line| match line {
            Ok(line) => !line.starts_with("{censored}"),
            Err(_) => true,
        })
        .map(|line| {
            let mut line = line?;
            let ws = line
                .find(' ')
                .expect("nix-store --gc --print-roots didn't have spaces in it, ridicolous");
            line.truncate(ws);
            Ok(line)
        })
        .filter(|path| match path {
            Ok(path) => can_delete(Path::new(path)).unwrap_or(false),
            Err(_) => true,
        })
        .collect::<io::Result<Vec<_>>>()?)
}
