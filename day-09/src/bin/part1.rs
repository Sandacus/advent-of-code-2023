use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 9!");
    let input = get_input("./src/data/input1.txt");
    let ans = part1(&input);
    println!("The answer is: {:?}", ans);
    // 2011053911
    // 2008960228
}

fn part1(input: &Vec<String>) -> i64 {
    println!("{:?}", input);
    let mut ans: Vec<i64> = Vec::new();
    for line in input {
        let string = line;

        let sensor_history: Vec<i64> = string
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        let diffs: Vec<i64> = sensor_history.clone();

        let last_vals: Vec<i64> = get_diffs(diffs);

        ans.push(last_vals.iter().sum());

    }
    // sum answers and return
    println!("{:?}", ans);
    ans.iter().sum::<i64>()
}

fn get_diffs(mut diffs: Vec<i64>) -> Vec<i64> {
    let mut last_vals: Vec<i64> = Vec::new();
    last_vals.push(diffs.last().copied().unwrap());
    while diffs.iter().sum::<i64>() != 0 {
        diffs = diffs
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        last_vals.push(diffs.last().copied().unwrap());
    }
    last_vals
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(path: &str) -> Vec<String> {
    let path = path;
    let contents = lines_from_file(path);
    // println!("File contents: {:?}", contents);
    contents
}