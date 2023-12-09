use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 9!");
    let input = get_input("./src/data/input1.txt");
    let ans = part2(&input);
    println!("The answer is: {:?}", ans);
    // 2011053911
    // 2008960228
}

fn part2(input: &Vec<String>) -> i64 {
    println!("{:?}", input);
    let mut ans: Vec<i64> = Vec::new();
    for line in input {
        let string = line;

        let sensor_history: Vec<i64> = string
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        let diffs: Vec<i64> = sensor_history.clone();

        let first_vals_revered: Vec<i64> = get_diffs(diffs);
        println!("first vals {:?}", first_vals_revered);

        // ans.push(first_vals.iter().map(|x| -x).sum::<i64>());
        let mut val: i64 = 0;
        for i in 1..first_vals_revered.len()-1 {
            val = first_vals_revered[i] - val;
        }
        ans.push(first_vals_revered.last().unwrap() - val);
    }
    // sum answers and return
    println!("{:?}", ans);
    ans.iter().sum::<i64>()
}

fn get_diffs(mut diffs: Vec<i64>) -> Vec<i64> {
    let mut first_vals: Vec<i64> = Vec::new();
    first_vals.push(diffs.first().copied().unwrap());
    while diffs.iter().sum::<i64>() != 0 {
        diffs = diffs
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        first_vals.push(diffs.first().copied().unwrap());
    }
    first_vals.into_iter().rev().collect()
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