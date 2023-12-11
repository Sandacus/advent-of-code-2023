use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::{Instant};


fn main() {
    println!("Hello, day 11!");
    let path = "./src/data/input1.txt";
    let input: Vec<Vec<char>> = get_input(path);
    let start = Instant::now();
    let ans = part2(input.clone());
    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed in expensive_function() is: {:?}s", duration);
    println!("The answer is: {:?}", ans); // too low
    // 82000210 - too low
    // 82000354 - too low
}

fn part2(input: Vec<Vec<char>>) -> i64 {
    // Expand galaxies!
    // Find rows that need expansion
    let mut rows_expansion: Vec<usize> = Vec::new();
    for i in 0..input.len() {
        if input[i].iter().all(|&c| c == '.') {
            rows_expansion.push(i);
        }
    }

    // Find columns that need expansion
    let mut cols_expansion: Vec<usize> = Vec::new();
    for j in 0..input[0].len() {
        let mut flag: bool = true;
        for i in 0..input.len() {
            if input[i][j] != '.' {
                flag = false;
                break;
            }
        }
        if flag { cols_expansion.push(j) }
    }

    // lets add rows and columns based on the expansion coefficient
    let expansion_coefficient: i64 = 1_000_000;

    println!("The rows for expansion are:\n{:?}", rows_expansion);
    println!("The cols for expansion are:\n{:?}", cols_expansion);

    // lets add in the expansion columns!
    let mut expanded_map: Vec<Vec<char>> = input.clone();

    let rows = expanded_map.len();
    let cols = expanded_map[0].len();
    // create a vector of coordinates of the galaxies
    let mut galaxies: Vec<[usize; 2]> = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            if expanded_map[i][j] == '#' {
                galaxies.push([i, j]);
            }
        }
    }

    // get the galaxy pairs
    let mut pairs: Vec<i64> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i != j {
                let mut x1: i64 = galaxies[i][0].clone() as i64;
                let mut x2: i64 = galaxies[j][0].clone() as i64;
                for row in rows_expansion.clone() { // row expansion
                    if galaxies[i][0] < row && galaxies[j][0] > row {
                        x2 += expansion_coefficient - 1;
                    } else if galaxies[i][0] > row && galaxies[j][0] < row {
                        x1 += expansion_coefficient - 1;
                    }
                }
                let x = (x2 - x1).abs(); // vertical (row) diff

                let mut y1: i64 = galaxies[i][1].clone() as i64;
                let mut y2: i64 = galaxies[j][1].clone() as i64;
                for col in cols_expansion.clone() { // column expansion
                    if galaxies[i][1] < col && galaxies[j][1] > col {
                        y2 += expansion_coefficient - 1;
                    } else if galaxies[i][1] > col && galaxies[j][1] < col {
                        y1 += expansion_coefficient - 1;
                    }
                }
                let y = (y2 - y1).abs(); // horizontal (column) diff

                pairs.push(x + y); // total distance X + y
            }
        }
    }
    println!("The expanded rows are {:?} and the expanded cols are: {:?}", rows, cols );
    println!("The number of pairs is: {:?}", pairs.len());
    println!("The total distance of all galaxy pairs is: {:?}", pairs.iter().sum::<i64>());

    pairs.iter().sum::<i64>()
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
    contents
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}