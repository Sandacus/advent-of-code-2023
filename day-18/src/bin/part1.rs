use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 18!");

    let path = "./src/data/test1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

#[derive(Debug)]
enum Dig {
    Dir(char),
    Num(i64),
}


fn part1(input: Vec<String>) -> i64 {
    // get the dig plan
    let dig_plan: Vec<(Dig, Dig)> = input.iter()
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            let dir: Dig = Dig::Dir(parts[0].chars().next().unwrap());
            let num: Dig = Dig::Num(parts[1].parse::<i64>().unwrap());
            (dir, num)
        })
        .collect();

    println!("The dig plan is: {:?}", dig_plan);


    42 // dummy return
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> Vec<String> {
    let path = path;
    let contents = lines_from_file(path);
    // println!("File contents: {:?}", contents);
    contents
}