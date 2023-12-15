use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 14!");

    let path = "./src/data/cycle3.txt";
    let input: Vec<Vec<char>> = get_input(path);
    // println!("The input is: {:?}", input);

    let ans = part1(input.clone());
    println!("The answer is: {:?}", ans);
}

fn part1(input: Vec<Vec<char>>) -> i64 {

    let tilt_map = input.clone();
    // tilt_map = tilt_rocks(input);

    let load = calculate_weight(&tilt_map);

    load as i64
}

fn tilt_rocks(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // loop over all columns
    for j in 0..m[0].len() {
        for i in 1..m.len() {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' upwards if space permits
                let mut idx = i - 1;
                while m[idx][j] == '.' {
                    m[idx][j] = 'O';
                    m[idx + 1][j] = '.';
                    if idx == 0 {break;}
                    idx -= 1;
                }
            }
        }
    }
    m
}

fn calculate_weight(m: &Vec<Vec<char>>) -> usize {
    // loop through matrix of rocks
    let mut load = 0;
    for i in 0..m.len() {
        let mut count = 0;
        for j in 0..m[0].len() {
            if m[i][j] == 'O' {
                count += 1;
            }
        }
        load += count * (m.len() - i);
        // println!("The score is: {:?}", count * (m.len() - i));
        // println!("for the line: {:?}", m[i]);
    }
    load
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(path: &str) -> Vec<Vec<char>> {
    let contents = lines_from_file(path);
    contents.iter()
        .map(|s| s.chars().collect())
        .collect()
}