
use std::fs::File;
use std::env;
use std::process::exit;
use std::io::{Read,Write};

fn main() {
    let dir = match env::current_dir() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("fatal: could not find current directory");
            exit(1);
        }
    };
    
    let mut git_dir = dir;
    while !git_dir.join(".git").is_dir() {
        let parent = match git_dir.parent() {
            Some(p) => p,
            None => {
                eprintln!("fatal: not a git repository (or any of the parent directories): .git");
                exit(1);
            }
        };
        env::set_current_dir(parent).expect("parent path operation should always succeed here");
        git_dir = parent.to_path_buf();
    }
    
    if env::args().len() == 1 {
        exit(0)
    }
    
    let mut ignores = vec![];
    {
        let mut contents = String::from("");
        let gitignore = File::open(".gitignore");
        if gitignore.is_ok() {
            let _ = gitignore.unwrap().read_to_string(&mut contents);
        }
        for line in contents.split('\n') {
            ignores.push(line.trim().to_owned())
        }
    }

    let gitignore = File::options().append(true).create(true).open(".gitignore");
    let mut gitignore = match gitignore {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: could not open .gitignore: {}", e);
            exit(1);
        }
    };

    for arg in env::args().skip(1) {
        if ignores.contains(&arg) {
            continue;
        }

        let res = gitignore.write_all((arg + "\n").as_bytes());
        if res.is_err() {
            eprintln!("fatal: could not complete write to .gitignore");
            exit(1);
        }
    }
}
