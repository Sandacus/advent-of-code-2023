use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 6!");

    let path = "./src/data/input.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part2(input);
    println!("The answer is: {:?}", ans); // 25_112, 24_815, 17_905 - too high 8491, 14444 not correct
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: (i64, i64)) -> i64 {

    let time = input.0;
    let distance = input.1;

    let scores: i64 = ways_to_win(time, distance);

    println!("The ways to win are: {:?}", scores);

    scores // return
}

fn ways_to_win(t: i64, d: i64) -> i64 {
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

fn get_input(path: &str) -> (i64, i64) {
    let path = path;
    let contents = lines_from_file(path);
    // convert to 2 vectors of times and distances
    let times: String = contents[0].split_whitespace()
        .skip(1)
        .collect();
    let distances: String = contents[1].split_whitespace()
        .skip(1)
        .collect();

    (times.parse::<i64>().unwrap(), distances.parse::<i64>().unwrap())
}
