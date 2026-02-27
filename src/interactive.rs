use nixgc_clear::error::{Error, Result};
use nixgc_clear::projects::Project;
use nixgc_clear::{Either, ask_or_parse};
use std::fs;

pub fn ask_and_remove(project: &mut Project) -> Result<()> {
    println!("Project: {:?}", project.root);
    println!("{} links", project.links.len());

    println!();
    loop {
        match ask_or_parse::<usize>("Delete links? [y/n/p/q/?]", &["y", "n", "p", "?", "q"])? {
            Either::Left(v) => match v {
                "y" => {
                    for i in project.links.iter() {
                        fs::remove_file(i)?;
                    }
                    break;
                }
                "n" => break,
                "p" => {
                    for (i, link) in project.links.iter().enumerate() {
                        println!("[{}] {}", i, link.to_string_lossy());
                    }
                }
                "?" => {
                    println!(
                        "y - delete all
n - do not delete any
p - print all links in project
number - delete link at index
q - quit
? - help"
                    );
                }
                "q" => return Err(Error::Quit),
                _ => println!("how'd you do that?"),
            },
            Either::Right(i) => {
                if i < project.links.len() {
                    fs::remove_file(project.links.swap_remove(i))?;
                }
            }
        }
    }

    Ok(())
}
