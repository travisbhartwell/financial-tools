use regex::RegexSet;
use std::io;
use std::io::prelude::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::exit,
};

fn load_regexes(regex_filename: &Path) -> RegexSet {
    let contents: String = std::fs::read_to_string(regex_filename).unwrap_or_else(|_| {
        panic!(
            "Problems reading from file: {}",
            regex_filename.to_str().unwrap()
        )
    });

    let regex_set: RegexSet = RegexSet::new(contents.lines()).unwrap();

    regex_set
}

fn main() {
    let regex_filename: PathBuf;

    if std::env::args().len() > 1 {
        regex_filename = PathBuf::from(std::env::args().nth(1).unwrap());
    } else {
        eprintln!("Missing filename argument for Regular Expression input.");
        exit(1);
    }

    let regex_set = load_regexes(&regex_filename);

    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut matches_file: File = File::create("matches.txt").unwrap();
    let mut non_matches_files: File = File::create("no-matches.txt").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();

        let matches: Vec<_> = regex_set.matches(line.as_str()).into_iter().collect();

        if matches.is_empty() {
            writeln!(non_matches_files, "{}", line).unwrap();
        }

        for m in matches {
            writeln!(
                matches_file,
                "Line: |{}| matched by |{}|",
                line,
                &regex_set.patterns()[m]
            )
            .unwrap();
        }
    }
}
