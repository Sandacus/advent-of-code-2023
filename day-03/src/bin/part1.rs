use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

const NOT_SYMBOLS: [char; 11] = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    println!("Hello day 3!");

    let input = get_input("src/data/input1.txt");
    println!("{:?}", input);

    let ans = part1(input);
    println!("The answer is: {}", ans); // 547977 incorrect
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


fn part1(input: Vec<Vec<char>>) -> i64 {
    let engine_schematic = input.clone();

    let part_numbers = find_part_nums(engine_schematic);

    part_numbers.iter().sum() // dummy return
}

fn find_part_nums(es: Vec<Vec<char>>) -> Vec<i64> {
    let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut val_nums: Vec<i64> = Vec::new();
    // filter out non-part numbers from engine schematic
    for i in 0..es.len() {
        let mut j_end = 0;
        for j in 0..es[0].len() {
            // skip to next number
            if j < j_end { continue; }
            // is es[i][j] a number?
            if numbers.contains(&es[i][j]) {
                let mut idx = j;
                let j_start = j;
                // are items next to it also numbers?
                while numbers.contains(&es[i][idx]) {
                    idx += 1;
                    if idx > es[i].len() - 1 { break; }
                }
                j_end = idx;
                // check number is valid
                let valid_number = check_number(i, j_start, j_end, &es);
                // build the number
                let num = build_number(i, j_start, j_end, &es);
                println!("Digit is: {} => validity: {}", num, valid_number);
                if valid_number {
                    val_nums.push(num);
                }
            }
        }
    }
    val_nums
}

fn build_number(row: usize, col_start: usize, col_end: usize, es: &Vec<Vec<char>>) -> i64 {
    let mut num_chars = vec!['0'; col_end - col_start];
    num_chars.clone_from_slice(&es[row][col_start..col_end]);

    let num_string: String = num_chars.into_iter().collect();
    println!("Inside build number. Num to build is: {}", num_string);

    let num: i64 = num_string.parse::<i64>().unwrap();
    num // return
}

fn check_number(row: usize, mut col_start: usize, col_end: usize, es: &Vec<Vec<char>>) -> bool {
    // assume false to start with and check for truth
    let mut number_check = false;

    // check left
    if col_start > 0 {
        let ch = es[row][col_start - 1];
        number_check = if !NOT_SYMBOLS.contains(&ch) { true } else { false }
    }
    if number_check { return number_check; }

    // check right
    if col_end < es[row].len() {
        let ch = es[row][col_end];
        println!("Inside check right. Character to right is: {}", ch);
        number_check = if !NOT_SYMBOLS.contains(&ch) { true } else { false }
    }
    if number_check { return number_check; }

    if col_start > 0 { col_start -= 1 } else { col_start = 0 } // guard against negative numbers
    // check above
    if row > 0 {
        for x in col_start..col_end + 1 {
            if x < es[row].len() {
                let ch = es[row - 1][x];
                number_check = if !NOT_SYMBOLS.contains(&ch) { true } else { false }
            }
            if number_check { break; }
        }
    }
    if number_check { return number_check; }

    // check below
    if row < es.len() - 1 {
        for x in col_start..col_end + 1 {
            if x < es[row].len() {
                let ch = es[row + 1][x];
                number_check = if !NOT_SYMBOLS.contains(&ch) { true } else { false }
            }
            if number_check { break; }
        }
    }

    number_check // return
}
