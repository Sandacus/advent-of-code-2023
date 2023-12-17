use std::ffi::c_ushort;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 16!");

    let input = get_input("src/data/test1.txt");
    // println!("{:?}", input);

    let ans = part1(input);
    println!("The answer is: {}", ans);
}

// define a struct for point with row, col, and direction
struct Point {
    row: usize,
    // row of matrix
    col: usize,         // column of matrix
}

fn part1(input: Vec<Vec<char>>) -> i64 {
    let (x_start, y_start): (usize, usize) = (0usize, 0usize);

    let mut light: Point = Point {
        row: x_start,
        col: y_start,
    };

    loop {
        // lookup direction based on point coords

        // use direction to update row and col


        // repeat
    }


    42 // dummy return
}

fn get_direction(row: usize, col: usize, map: &Vec<Vec<char>>) -> char {
    map[row][col]
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