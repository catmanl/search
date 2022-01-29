# search
Minimal grep-like program

It does not support looking through folders or more than two files at once. It also does not support this kind of syntax:
```
$ cat file | grep value
```

## Usage
```
$ search <file> <pattern>
```
It will look through the file and find all lines including `pattern`

## Building
```
$ rustc search.rs
```

## Example
```
./search search.rs a

# Output
search.rs:2 | use std::io::{self, BufRead};
search.rs:5 | fn main() -> std::io::Result<()> {
search.rs:6 |     fail_if(env::args().len() < 3, "Usage: search <file> <pattern>");
search.rs:7 |     fail_if(env::args().len() > 3, "Usage: search <file> <pattern>");
search.rs:9 |     let first = format!("{}", env::args().nth(1).unwrap());
search.rs:10 |     let file = format!("{}", env::args().nth(1).unwrap()); // to use later
search.rs:11 |     let search = format!("{}", env::args().nth(2).unwrap());
search.rs:16 |     for i in io::BufReader::new(f).lines() {
search.rs:22 |         if line.as_ref().unwrap().contains(&search) {
search.rs:23 |             println!("{}:{} | {}", file, index, line.unwrap());
search.rs:30 | fn fail_if(condition: bool, fail_msg: &str) {
search.rs:32 |         println!("{}", fail_msg);
```

