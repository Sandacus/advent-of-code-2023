use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 13!");

    let path = "./src/data/test1.txt";
    let input: Vec<Vec<String>> = get_input(path);
    // println!("The input is: {:?}", input);

    let ans = part1(input.clone());
    println!("The answer is: {:?}", ans); // 1203, 3548, 27807 - too low
}

fn part1(input: Vec<Vec<String>>) -> i64 {

    // find rows reflections
    let mut score = 0;
    for pattern in input {
        // column reflections
        let cols = get_cols_in_vec(&pattern);
        score += find_reflection(cols);

        // row reflections
        score += find_reflection(pattern) * 100;
    }

    score
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

fn find_reflection(input: Vec<String>) -> i64 {
    let mut reflection_idx = 0usize;
    for (i, line) in input.iter().enumerate() {
        if i == 0 {continue} // skip first line
        if line == &input[i-1] {
            reflection_idx = i - 1;
            break;
        }
    }

    // check if perfect mirror
    // work out how many indexes to loop over
    let mut depth = 0;
    if reflection_idx < input.len()/2 {
        depth = reflection_idx+1;
    } else {
        depth = input.len()-1 - reflection_idx;
    }
    // loop through matching pairs over depth
    let mut flag = false;
    for i in 0..depth {
        if input[reflection_idx-i] != input[reflection_idx+i+1] {
            flag = false;
            break;
        }
        flag = true;
    }
    let mut score = 0;
    if flag {
        score = reflection_idx+1;
    }

    // // find number of rows
    // let rows = input.len();
    // let mut score = 0;
    //
    // if rows % 2 == 0 {
    //     if input[rows / 2] == input[rows / 2 - 1] {
    //         score = rows / 2 - 1;
    //     }
    // } else {
    //     // middle and lower line
    //     if input[rows / 2] == input[rows / 2 - 1] {
    //         let mut reflection_flag = true;
    //         // check outer rows
    //         for i in 1..rows / 2 {
    //             if input[rows / 2 + i] != input[rows / 2 - 1 - i] {
    //                 reflection_flag = false;
    //                 break;
    //             }
    //         }
    //         if reflection_flag {
    //             score += rows / 2;
    //         }
    //     }
    //     // middle and upper line
    //     if input[rows / 2] == input[rows / 2 + 1] {
    //         let mut reflection_flag = true;
    //         // check outer rows
    //         for i in 1..rows / 2 {
    //             if input[rows / 2 - i] != input[rows / 2 + 1 + i] {
    //                 reflection_flag = false;
    //                 break;
    //             }
    //         }
    //         if reflection_flag {
    //             score += rows / 2 + 1;
    //         }
    //     }
    // }

    score as i64
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