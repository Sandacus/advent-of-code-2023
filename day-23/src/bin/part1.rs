use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 23!");

    let path = "./src/data/test1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: Vec<Vec<char>>) -> i64 {
    // println!("Here is the input {:?}", input);

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut p: (usize, usize) = (0, 1);

    let mut dir = '_'; // initialise direction
    loop {
        let start_length = path.len();
        loop {
            let p_start = p.clone();
            println!("The current point is {:?} and direction is {}", p, dir);
            // look north
            if look_north(&p, &input) && dir != 'S' {
                path.push(p.clone());
                p.0 -= 1;
                dir = 'N';
                break;
            }
            // look east
            if look_east(&p, &input) && dir != 'W' {
                path.push(p.clone());
                p.1 += 1;
                dir = 'E';
                break;
            }
            // look south
            if look_south(&p, &input) && dir != 'N' {
                path.push(p.clone());
                p.0 += 1;
                dir = 'S';
                break;
            }
            // look west
            if look_west(&p, &input) && dir != 'E' {
                path.push(p.clone());
                p.1 -= 1;
                dir = 'W';
                break;
            }
            // break if point not changed
            if p_start == p {
                break;
            }
        }

        if start_length == path.len() || path.contains(&p) {
            break;
        }
    }

    println!("The path is: {:?}", path);
    println!("The next point is {:?}", p);

    path.len() as i64
    // 42 // dummy return
}

fn look_north(p: &(usize, usize), maze: &Vec<Vec<char>>) -> bool {
    if p.0 == 0 {
        return false;
    }
    match maze[p.0-1][p.1] {
        '.' => true,
        '#' => false,
        _ => true,
    }
}

fn look_east(p: &(usize, usize), maze: &Vec<Vec<char>>) -> bool {
    if p.1 == maze[0].len()-1 {
        return false;
    }
    match maze[p.0][p.1+1] {
        '.' => true,
        '#' => false,
        _ => true,
    }
}

fn look_south(p: &(usize, usize), maze: &Vec<Vec<char>>) -> bool {
    if p.0 == maze.len()-1 {
        return false;
    }
    match maze[p.0+1][p.1] {
        '.' => true,
        '#' => false,
        _ => true,
    }
}

fn look_west(p: &(usize, usize), maze: &Vec<Vec<char>>) -> bool {
    if p.1 == 0 {
        return false;
    }
    match maze[p.0][p.1-1] {
        '.' => true,
        '#' => false,
        _ => true,
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> Vec<Vec<char>> {
    let path = path;
    let contents = lines_from_file(path);
    // println!("File contents: {:?}", contents);
    contents.iter()
        .map(|s| s.chars().collect())
        .collect()
}