use std::env::args;
use std::fs;
use std::io::{BufRead, Write, Error};
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;

fn copy_file(from: &Path, to: &str, patterns: &Vec<(Regex, String)>) {
    let file = match fs::read_to_string(from) {
        Err(a) => {
            let res = fs::copy(from, to);
            match res {
                Err(e) => eprintln!("{e}"),
                Ok(v) => {}
            }
            return;
        },
        Ok(a) => a
    };
    let mut new_file = fs::File::create(to).unwrap();
    let mut new_line: String;
    for line in file.lines() {
        new_line = line.to_string();
        if !line.starts_with("//") && !line.starts_with('#') {
            for (regex, replace) in patterns {
                let pos = regex.find(line);
                if pos.is_none() || pos.unwrap().start() == 0 {
                    continue;
                }
                new_line = regex.replace_all(&new_line, replace).to_string();
            }
        }
        writeln!(new_file, "{}", &*new_line).expect("Failed to write");
    }
}



fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprint!("This program requires 2 arguments:\n1)input directory;\n2)output directory NAME");
        std::process::exit(1);
    }
    let base_path = Path::new(&args[1]);
    let base_path_str = base_path.file_name().unwrap().to_str().unwrap();
    let new_path_str = Path::new(&args[2]).to_str().unwrap();

    let mut patterns_path: Vec<(Regex, String)> = Vec::new();
    let mut patterns_file: Vec<(Regex, String)> = Vec::new();
    let pattern_string = std::fs::read_to_string("patterns.txt").unwrap();
    for line in pattern_string.lines() {
        let space = line.find("  ").unwrap();
        let from = &line[..space].replace('*', "\\\\");
        let to = line[space+2..].replace('*', "\\");
        patterns_path.push((Regex::new(from).unwrap(), to));
        let from = &line[..space].replace('*', ".");
        let to = line[space+2..].replace('*', ".");
        patterns_file.push((Regex::new(from).unwrap(), to));
    }
    patterns_path.push((Regex::new(base_path_str).unwrap(), String::from(new_path_str)));


    let base_path_abs = fs::canonicalize(base_path).unwrap();
    for entry in WalkDir::new(base_path).into_iter().filter_map(Result::ok) {
        let mut path = entry.path().to_str().unwrap().to_string();
        for pattern in &patterns_path {
            path = pattern.0.replace_all(&path, &pattern.1).to_string();
        }
        println!("{}", path);
        if entry.file_type().is_file() {
            copy_file(entry.path(), &path, &patterns_file);

        } else if entry.file_type().is_dir() {
            std::fs::create_dir_all(&path).expect(&format!("Unable to create dir {path}"));
        }
    }
}
