use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() -> std::io::Result<()> {
    fail_if(env::args().len() < 3, "Usage: search <file> <pattern>");
    fail_if(env::args().len() > 3, "Usage: search <file> <pattern>");

    let first = format!("{}", env::args().nth(1).unwrap());
    let file = format!("{}", env::args().nth(1).unwrap()); // to use later
    let search = format!("{}", env::args().nth(2).unwrap());
    let f = File::open(first)?;
    let mut lines = vec![];
    let mut index: i64 = 0;

    for i in io::BufReader::new(f).lines() {
        lines.push(i);
    }

    for line in lines {
        index += 1;
        if line.as_ref().unwrap().contains(&search) {
            println!("{}:{} | {}", file, index, line.unwrap());
        }
    }

    Ok(())
}

fn fail_if(condition: bool, fail_msg: &str) {
    if condition {
        println!("{}", fail_msg);
        std::process::exit(1);
    }
}

