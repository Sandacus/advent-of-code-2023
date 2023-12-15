use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 15!");

    let path = "./src/data/input1.txt";
    let input: Vec<char> = get_input(path);
    println!("The input is: {:?}", input);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: Vec<char>) -> u32 {

    let mut val: u32 = 0;
    let mut vec_vals = Vec::new();
    for ch in input {
        if ch == ',' {
            vec_vals.push(val);
            val = 0;
        } else {
            val = get_hash(ch, val);
            println!("Current value is: {:?}", val);
        }
    }
    vec_vals.push(val); // get last value

    vec_vals.iter().sum() // dummy return
}

fn get_hash(mut ch: char, mut val: u32) -> u32 {
    // determine ASCII code for character
    println!("The ASCII code for {} is {}", ch, ch as u32);
    // add ASCII code to current value
    val += ch as u32;

    // multiply current value by 17
    val *= 17;

    // modulo current value % 256
    val = val%256;

    val // return
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(path: &str) -> Vec<char> {
    let contents = lines_from_file(path);
    let string = contents[0].clone();
    string.chars().collect()

    // contents.iter()
    //     .map(|s| s.chars().collect())
    //     .collect()
}