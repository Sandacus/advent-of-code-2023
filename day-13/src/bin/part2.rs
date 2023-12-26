use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 13!");

    let path = "./src/data/input1.txt";
    let input: Vec<Vec<String>> = get_input(path);
    // println!("The input is: {:?}", input);

    let ans = part2(input.clone());
    println!("The answer is: {:?}", ans); // 1203, 3548, 20527, 27807 - too low
}

fn part2(input: Vec<Vec<String>>) -> i64 {
    // find rows reflections
    let mut score = 0;

    find_smudge();

    // column reflections
    for pattern in &input {
        let cols = get_cols_in_vec(&pattern);
        score += find_reflection(&cols);
    }
    println!("The score after the columns is: {:?}", score);

    // row reflections
    for pattern in &input {
        score += find_reflection(pattern) * 100;
    }
    println!("The score after the rows is: {:?}", score);

    score
}

fn find_smudge(mir: Vec<String>) {
    // Find the 1 smudge in the mirror.
    // Go row by row and compare the string to the one below and if there is a total of one
    // this is the smudge. Then swap the character on one of the lines from '.' to '#'.
    let length = mir.len() - 1;

    for i in 0..length {

    }
}

fn get_cols_in_vec(pattern: &Vec<String>) -> Vec<String> {
    // convert the vector to a new vector of rows: transpose!
    // make a vec of vec of chars
    let vec_of_vec: Vec<Vec<char>> = pattern.iter()
        .map(|s| s.chars().collect())
        .collect();

    // transpose
    let mut vec_of_cols: Vec<String> = Vec::new();
    let mut col_chars_vec = Vec::new();
    for j in 0..vec_of_vec[0].len() {
        for i in 0..vec_of_vec.len() {
            col_chars_vec.push(vec_of_vec[i][j]);
        }
        vec_of_cols.push(col_chars_vec.into_iter().collect());
        col_chars_vec = Vec::new();
    }
    vec_of_cols
}

fn find_reflection(input: &Vec<String>) -> i64 {
    // start checking for reflections
    // if reflection continue moving up one and down and checking that until one or both get to either end
    let length = input.len() - 1;
    let mut reflection = 0;

    for i in 0..length {
        if input[i] == input[i+1] {
            let mut j = i;
            let mut k = i + 1;
            while input[j] == input[k] {
                if j == 0 || k == length {
                    reflection += i + 1;
                    break;
                }
                j -= 1;
                k += 1;
            }
        }
    }

    reflection as i64
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> Vec<Vec<String>> {
    let contents = lines_from_file(path);
    let mut inputs: Vec<Vec<String>> = Vec::new();
    let mut pattern: Vec<String> = Vec::new();
    for line in contents {
        if line == "" {
            inputs.push(pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line);
        }
    }
    inputs.push(pattern); // catch last vector built in for loop

    inputs
}