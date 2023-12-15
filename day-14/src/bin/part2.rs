use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 14!");

    let path = "./src/data/test1.txt";
    let input: Vec<Vec<char>> = get_input(path);
    // println!("The input is: {:?}", input);

    let start = Instant::now();
    let ans = part2(input.clone());
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: Vec<Vec<char>>) -> i64 {
    let mut tilt_map = input;
    let cycles = 1;
    let mut load: usize = 0;
    let mut previous_load = 0;
    for i in 0..cycles {
        tilt_map = tilt_north(tilt_map);
        tilt_map = tilt_west(tilt_map);
        tilt_map = tilt_south(tilt_map);
        tilt_map = tilt_east(tilt_map);
        load = calculate_weight(&tilt_map);
        println!("Cycle number: {:?} => North load: {}", i+1, load);

        // if load == previous_load {
        //     break;
        // }
        previous_load = load;
    }

    for line in &tilt_map {
        println!("{:?}", line);
    }

    // let load = calculate_weight(&tilt_map);

    load as i64
}

fn tilt_east(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..m.len() {
        for j in m[0].len()-1..0 {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' east if space permits
                let mut idx = j - 1;
                while m[i][idx] == '.' {
                    m[i][idx] = '0';
                    m[i][idx + 1] = '.';
                    if idx == 0 { break; }
                    idx -= 1;
                }
            }
        }
    }
    m // return
}

fn tilt_south(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // loop over all columns
    for j in 0..m[0].len() {
        for i in m.len()-1..0 {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' south if space permits
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

fn tilt_west(mut m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..m.len() {
        for j in 1..m[0].len() {
            if m[i][j] == 'O' {
                // while loop for keep moving '0' west if space permits
                let mut idx = j - 1;
                while m[i][idx] == '.' {
                    m[i][idx] = '0';
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