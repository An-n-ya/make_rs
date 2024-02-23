mod lexer;

use std::{fs::File, io::Read};

use clap::Parser;
use lexer::Lexer;

#[derive(Parser)]
struct Args {
    target: Option<String>,
}
fn main() {
    // parse arg
    let args = Args::parse();

    // look for makefile
    let makefile = look_for_makefile();

    // preprocess makefile
    preprocess_makefile(makefile);

    // parse makefile

    // run target and corresponding recipe
}

fn look_for_makefile() -> File {
    const CANDIDATE: [&str; 3] = ["GNUmakefile", "makefile", "Makefile"];

    for m in CANDIDATE {
        if std::path::Path::new(m).exists() {
            return File::open(m).ok().unwrap();
        }
    }
    panic!("No makefile found. Stop");
}

fn preprocess_makefile(mut file: File) -> Lexer {
    let mut s = "".to_string();
    file.read_to_string(&mut s).unwrap();
    // process `new line` operator
    let mut res = vec![];
    let mut lines = s.lines();
    while let Some(mut line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let mut cur_line: String = "".to_string();
        while line.ends_with("\\") {
            cur_line += line.strip_suffix("\\").unwrap();
            if let Some(new_line) = lines.next() {
                line = new_line;
            } else {
                panic!("last line cannot end with `\\`");
            }
        }
        cur_line += line;
        res.push(cur_line);
    }
    let res = res.join("\n");
    Lexer::new(res)
}
