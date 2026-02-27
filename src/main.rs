use nixgc_clear::{
    ask,
    nix::get_gc_roots,
    projects::{Projects, split_paths},
};
use std::{fs, path::Path};

mod interactive;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let roots = get_gc_roots()?;
    let paths = roots.iter().map(Path::new).collect::<Vec<_>>();

    let Projects {
        projects,
        mut no_project,
    } = split_paths(paths.as_slice())?;

    no_project.retain(|p| {
        !p.components().any(|s| {
            let s = s.as_os_str();
            (s == ".local") || (s == ".cache")
        })
    });

    for mut i in projects {
        interactive::ask_and_remove(&mut i)?;
        println!();
    }
    if !no_project.is_empty() {
        println!("Stray links:");
        for i in no_project {
            println!("{:?}", i);
            if ask("Delete? [y/n]", &["y", "n"])? == "y" {
                fs::remove_file(i)?;
            }
        }
    }

    if ask("Run nix-collect-garbage? [y/n]", &["y", "n"])? == "y" {
        return Err(nixgc_clear::unix::execvp_safe(c"nix-collect-garbage", &[]).into());
    }

    Ok(())
}
