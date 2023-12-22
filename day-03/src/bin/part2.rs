use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

const GEAR: [char; 1] = ['*'];

fn main() {
    println!("Hello day 3!");

    let input = get_input("src/data/input1.txt");
    println!("{:?}", input);

    let ans = part2(input);
    println!("The answer is: {}", ans);
}

fn part2(input: Vec<Vec<char>>) -> i64 {
    let engine_schematic = input.clone();

    let gears: Vec<(usize, usize)> = find_gear(&engine_schematic);
    println!("The gears are: {:?}", gears);

    let part_numbers = find_part_nums(engine_schematic);
    println!("Part numbers: {:?}", part_numbers);

    let gear_ratios: Vec<Vec<i64>> = check_gear_ratios(gears, part_numbers);
    println!("gear ratios: {:?}", gear_ratios);

    let mut ans: i64 = 0;
    for gr in gear_ratios {
        ans += gr.iter().product::<i64>();
    }

    ans //
}

fn check_gear_ratios(gears: Vec<(usize, usize)>, part_numbers: HashMap<(usize, usize, usize), i64>) -> Vec<Vec<i64>> {
    // define vector for putting in gear ratio pairs
    let mut gear_ratios: Vec<Vec<i64>> = Vec::new();

    // go though gears and check how many numbers are adjacent
    for gear in gears {
        let mut count = 0;
        let mut nums: Vec<i64> = Vec::new();
        for (key, val) in &part_numbers {
            // check same row left or right
            if gear.0 == key.0 {
                // gear at start
                if gear.1 == 0 {
                    // just check right
                    if key.1 == 1 {
                        count += 1;
                        nums.push(val.clone()); // add number to nums vector
                        println!("Count = {}", count);
                    }
                } else if gear.1 == key.2 || gear.1+1 == key.1 {
                    count += 1;
                    nums.push(val.clone()); // add number to nums vector
                    println!("Count = {}", count);
                }
            }
            // check row above
            if gear.0 == key.0+1 {
                if gear.1 == 0 {
                    if key.1 <= 1 {
                        count += 1;
                        nums.push(val.clone()); // add number to nums vector
                        println!("Count = {}", count);
                    }
                } else if gear.1 == key.2 || gear.1+1 == key.1 {
                    count += 1;
                    nums.push(val.clone()); // add number to nums vector
                    println!("Count = {}", count);
                } else if gear.1 <= key.2 && gear.1+1 >= key.1 {
                    count += 1;
                    nums.push(val.clone()); // add number to nums vector
                    println!("Count = {}", count);
                }
            }
            // check row below
            if gear.0+1 == key.0 {
                if gear.1 == 0 {
                    if key.1 <= 1 {
                        count += 1;
                        nums.push(val.clone()); // add number to nums vector
                        println!("Count = {}", count);
                    }
                } else if gear.1 == key.2 || gear.1+1 == key.1 {
                    count += 1;
                    nums.push(val.clone()); // add number to nums vector
                    println!("Count = {}", count);
                } else if gear.1 <= key.2 && gear.1+1 >= key.1 {
                    count += 1;
                    nums.push(val.clone()); // add number to nums vector
                    println!("Count = {}", count);
                }
            }
        }
        if count == 2 {
            gear_ratios.push(nums);
            println!("valid gear ratio!");
        }
    }
    gear_ratios
}

fn find_gear(es: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    // find gears '*'
    let mut gears: Vec<(usize, usize)> = Vec::new();
    let rows = es.len()-1;
    let cols = es[0].len()-1;
    for i in 0..rows {
        for j in 0..cols {
            if es[i][j] == '*' {
                gears.push((i, j));
            }
        }
    }
    gears
}

fn find_part_nums(es: Vec<Vec<char>>) -> HashMap<(usize, usize, usize), i64> {
    let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // create hashmap of number and tuple with (row, cols_start, col_end)
    let mut val_nums: HashMap<(usize, usize, usize), i64> = HashMap::new();
    let mut num_vec: Vec<i64> = Vec::new();
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
                    num_vec.push(num);
                    val_nums.insert((i, j_start, j_end), num);
                }
            }
        }
    }
    println!("The part numbers are: {:?}", num_vec);
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
        number_check = if GEAR.contains(&ch) { true } else { false }
    }
    if number_check { return number_check; }

    // check right
    if col_end < es[row].len() {
        let ch = es[row][col_end];
        // println!("Inside check right. Character to right is: {}", ch);
        number_check = if GEAR.contains(&ch) { true } else { false }
    }
    if number_check { return number_check; }

    if col_start > 0 { col_start -= 1 } else { col_start = 0 } // guard against negative numbers
    // check above
    if row > 0 {
        for x in col_start..col_end + 1 {
            if x < es[row].len() {
                let ch = es[row - 1][x];
                number_check = if GEAR.contains(&ch) { true } else { false }
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
                number_check = if GEAR.contains(&ch) { true } else { false }
            }
            if number_check { break; }
        }
    }

    number_check // return
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