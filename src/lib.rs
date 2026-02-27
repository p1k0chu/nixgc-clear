pub mod error;
pub mod fs;
pub mod nix;
pub mod projects;
pub mod unix;

use error::Result;
use std::{
    io::{self, Stdin, Stdout, Write},
    str::FromStr,
};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn ask<'a>(msg: &str, options: &[&'a str]) -> Result<&'a str> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut buffer = String::new();
    Ok(loop {
        let sel = _ask(&mut stdout, &stdin, &mut buffer, msg)?;
        if let Some(sel) = options.iter().find(|s| sel == **s) {
            break *sel;
        }
    })
}

pub fn ask_or_parse<'a, T>(msg: &str, options: &[&'a str]) -> Result<Either<&'a str, T>>
where
    T: FromStr,
{
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut buffer = String::new();
    Ok(loop {
        let sel = _ask(&mut stdout, &stdin, &mut buffer, msg)?;
        if let Some(sel) = options.iter().find(|s| sel == **s) {
            break Either::Left(*sel);
        } else if let Ok(v) = sel.parse::<T>() {
            break Either::Right(v);
        }
    })
}

fn _ask<'a>(
    stdout: &mut Stdout,
    stdin: &Stdin,
    buffer: &'a mut String,
    msg: &str,
) -> Result<&'a str> {
    print!("{} ", msg);
    stdout.flush()?;

    buffer.clear();
    stdin.read_line(buffer)?;
    Ok(buffer.trim())
}
