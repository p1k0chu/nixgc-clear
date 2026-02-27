use crate::error::Result;
use std::{collections::HashMap, io::ErrorKind, path::Path};

pub struct Projects<'a> {
    pub projects: Vec<Project<'a>>,
    pub no_project: Vec<&'a Path>,
}

pub struct Project<'a> {
    /// project root.
    pub root: &'a Path,
    /// gc roots, symlinks.
    pub links: Vec<&'a Path>,
}

pub fn split_paths<'a>(paths: &[&'a Path]) -> Result<Projects<'a>> {
    let mut projects: HashMap<&'a Path, Vec<&'a Path>> = HashMap::new();
    let mut noproject: Vec<_> = Vec::new();

    for path in paths.iter() {
        let proj_root = find_project_root(path)?;
        if let Some(proj_root) = proj_root {
            let proj = projects.entry(proj_root).or_default();
            proj.push(*path);
        } else {
            noproject.push(*path);
        }
    }

    Ok(Projects {
        projects: projects
            .into_iter()
            .map(|(k, v)| Project { root: k, links: v })
            .collect(),
        no_project: noproject,
    })
}

/// finds .direnv or .git
pub fn find_project_root(path: &Path) -> Result<Option<&Path>> {
    // take parent immediately. symlinks can point to a project.
    let Some(mut path) = path.parent() else {
        return Ok(None);
    };
    Ok(Some(loop {
        match std::fs::read_dir(path) {
            Ok(mut v) => {
                if v.any(|i| {
                    i.is_ok_and(|i| {
                        let name = i.file_name();
                        (name == ".direnv" || name == ".git")
                            && i.file_type().is_ok_and(|t| t.is_dir())
                    })
                }) {
                    break path;
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::NotADirectory => (),
                _ => return Err(err.into()),
            },
        }
        path = match path.parent() {
            Some(v) => v,
            _ => return Ok(None),
        };
    }))
}
