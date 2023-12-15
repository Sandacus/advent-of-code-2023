use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 14!");

    let path = "./src/data/input1.txt";
    let input: Vec<Vec<char>> = get_input(path);
    // println!("The input is: {:?}", input);

    let start = Instant::now();
    let ans = part2(input.clone());
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: Vec<Vec<char>>) -> i64 {
    let mut tilt_map = input.clone();


    let cycles = 10_000;

    let mut load: usize = 0;
    let mut previous_loads: Vec<usize> = Vec::new();
    let mut cycle_count = 0;
    for i in 0..cycles {
        cycle_count += 1;
        tilt_map = tilt_north(tilt_map);
        tilt_map = tilt_west(tilt_map);
        tilt_map = tilt_south(tilt_map);
        tilt_map = tilt_east(tilt_map);
        load = calculate_weight(&tilt_map);
        println!("Cycle number: {:?} => North load: {}", i+1, load);

        // don't have to loop to 1_000_000_000 !!!!
        // see if there is a cycle and when you get a factor of 1_000_000_000
        // and have seen the load before you know the load
        if (previous_loads.contains(&load)) && (1_000_000_000 % cycle_count == 0) {
            println!("Cycle count = {}", cycle_count);
            break;
        }
        previous_loads.push(load);
    }

    // visualise result
    // for line in &tilt_map {
    //     println!("{:?}", line);
    // }

    load as i64
}

fn tilt_east(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..m.len() {
        for j in (0..m[0].len()-1).rev() {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' east if space permits
                let mut idx = j;
                while m[i][idx+1] == '.' {
                    m[i][idx+1] = 'O';
                    m[i][idx] = '.';
                    if idx == m[0].len()-2 { break; }
                    idx += 1;
                }
            }
        }
    }
    m // return
}

fn tilt_south(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // loop over all columns
    for j in 0..m[0].len() {
        for i in (0..m.len()-1).rev() {

            if m[i][j] == 'O' {
                // while loop for keep moving '0' south if space permits
                let mut idx = i+1;
                while m[idx][j] == '.' {
                    m[idx][j] = 'O';
                    m[idx-1][j] = '.';
                    if idx >= m.len()-1 { break; }
                    idx += 1;
                }
            }
        }
    }
    m // return
}

fn tilt_west(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..m.len() {
        for j in 1..m[0].len() {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' west if space permits
                let mut idx = j - 1;
                while m[i][idx] == '.' {
                    m[i][idx] = 'O';
                    m[i][idx + 1] = '.';
                    if idx == 0 { break; }
                    idx -= 1;
                }
            }
        }
    }
    m // return
}

fn tilt_north(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // loop over all columns
    for j in 0..m[0].len() {
        for i in 1..m.len() {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' north if space permits
                let mut idx = i - 1;
                while m[idx][j] == '.' {
                    m[idx][j] = 'O';
                    m[idx + 1][j] = '.';
                    if idx == 0 { break; }
                    idx -= 1;
                }
            }
        }
    }
    m // return
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