use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 11!");

    let path = "./src/data/input1.txt";
    let input: Vec<Vec<char>> = get_input(path);

    let ans = part2(input.clone());
    // 10228048 > too low
    // 10228230
    println!("The answer is: {:?}", ans);
}

fn part2(input: Vec<Vec<char>>) -> usize {
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

    println!("The rows for expansion are:\n{:?}", rows_expansion);
    println!("The cols for expansion are:\n{:?}", cols_expansion);

    // lets add in the expansion columns!
    let mut expanded_map: Vec<Vec<char>> = input.clone();
    let row_exp = input[rows_expansion[0]].clone(); // copy a row of "."
    for (i, idx) in rows_expansion.iter().enumerate() {
        expanded_map.insert(i+idx, row_exp.clone());
    }
    for (j, idx) in cols_expansion.iter().enumerate() {
        for i in 0..expanded_map.len() {
            expanded_map[i].insert(j+idx, '*');
        }
    }

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
    // println!("The coordinates of the galaxies are:\n{:?}", galaxies);

    // get the galaxy pairs
    // let list = vec![1, 2 ,3];
    let mut pairs: Vec<usize> = Vec::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            if i != j {
                let x = (galaxies[j][0]).abs_diff(galaxies[i][0]); // horizontal diff
                let y = (galaxies[j][1]).abs_diff(galaxies[i][1]); // vertical diff
                pairs.push(x + y); // get the ceiling number
            }
        }
    }
    println!("The expanded rows are {:?} and the expanded cols are: {:?}", rows, cols );
    println!("The number of pairs is: {:?}", pairs.len());
    println!("The total distance of all galaxy pairs is: {:?}", pairs.iter().sum::<usize>());

    // for row in expanded_map {
    //     println!("{:?}", row);
    // }

    pairs.iter().sum::<usize>()
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