use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 6!");

    let path = "./src/data/input.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans); // 25_112, 24_815, 17_905 - too high 8491, 14444 not correct
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {

    let times = input.0;
    let distances = input.1;
    let length = times.len();

    let mut scores: Vec<i32> = Vec::new();
    for i in 0..length {
        scores.push(ways_to_win(times[i], distances[i]));
    }

    println!("The ways to win are: {:?}", scores);

    scores.iter().product() // return
}

fn ways_to_win(t: i32, d: i32) -> i32 {
    // s = v*t
    let mut count = 0;
    for i in 1..t {
        let dist = i * (t - i);
        if dist > d {
            count += 1;
        }
    }

    count // dummy
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let path = path;
    let contents = lines_from_file(path);
    // convert to 2 vectors of times and distances
    let times = contents[0].split_whitespace()
        .skip(1)
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let distances = contents[1].split_whitespace()
        .skip(1)
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    (times, distances)
}
