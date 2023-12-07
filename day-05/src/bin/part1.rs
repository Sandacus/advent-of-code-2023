use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, day 5!");

    let input = get_input();
    let nums: Vec<_> = input
        .into_iter()
        .filter_map(|s| s.replace(" ", "").parse::<i64>().ok())
        .collect();
    println!("{:?}", nums);
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input() -> Vec<String> {
    let path = "./src/bin/test1.txt";
    let contents = lines_from_file(path);
    println!("File contents: {:?}", contents);

    contents
}