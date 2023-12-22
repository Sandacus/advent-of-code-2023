use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 21!");
    let path = "./src/data/input1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn part1(input: Vec<Vec<char>>) -> i64 {
    // find start position, S
    let mut start = Point {x: 0, y: 0};
    for (i, v) in input.iter().enumerate() {
        for (j, ch) in v.iter().enumerate() {
            if ch == &'S' {
                start = Point { x: i, y: j };
                break;
            }
        }
    }
    println!("Start point: {:?}", start);
    // initialise vector of current positions
    let mut new_points: Vec<Point> = Vec::new();
    let mut plots: Vec<Point> = vec![start];

    let steps: usize = 64;
    // loop through steps
    for step in 0..steps {
        new_points.clear();
        // loop through current locations
        for loc in &plots {
            // for each location check north, south, east and west and collect new valid points
            if check_north(&loc, &input) {
                let p_n = get_point(&loc, 'n');
                if !new_points.contains(&p_n) {
                    new_points.push(p_n.clone());
                    println!("in check north: {:?}", p_n);
                }

            }
            if check_south(&loc, &input) {
                let p_s = get_point(&loc, 's');
                if !new_points.contains(&p_s) {
                    new_points.push(p_s.clone());
                    println!("in check south: {:?}", p_s);
                }

            }
            if check_east(&loc, &input) {
                let p_e = get_point(&loc, 'e');
                if !new_points.contains(&p_e) {
                    new_points.push(p_e.clone());
                    println!("in check east: {:?}", p_e);
                }

            }
            if check_west(&loc, &input) {
                let p_w = get_point(&loc, 'w');
                if !new_points.contains(&p_w) {
                    new_points.push(p_w.clone());
                    println!("in check west: {:?}", p_w);
                }
            }
        }
        println!("New points after {} steps are: {:?}",step+1, new_points);
        plots = new_points.clone();
        println!("The plots after {} steps are: {:?}",step+1, plots);
    }
    println!("The number of plots after {} steps is: {:?}",steps, plots.len());

    plots.len() as i64 // return
}

fn get_point(p: &Point, dir: char) -> Point {
    if dir == 'n' {
        let x_n = p.x.clone() - 1;
        let y_n = p.y.clone();
        return Point{ x: x_n, y: y_n };
    }
    if dir == 's' {
        let x_s = p.x.clone() + 1;
        let y_s = p.y.clone();
        return Point{ x: x_s, y: y_s };
    }
    if dir == 'e' {
        let x_e = p.x.clone();
        let y_e = p.y.clone() + 1;
        return Point{ x: x_e, y: y_e };
    }
    if dir == 'w' {
        let x_w = p.x.clone();
        let y_w = p.y.clone() - 1;
        return Point{ x: x_w, y: y_w };
    }
    p.clone()
}

fn check_north(p: &Point, map: &Vec<Vec<char>>) -> bool {
    let mut valid_move = false;
    if p.x > 0 {
        match map[p.x-1][p.y] {
            '.' => valid_move = true,
            '#' => valid_move = false,
            'S' => valid_move = true,
            _ => panic!("==> No point found in check_north! <==")
        }
    }
    valid_move
}

fn check_south(p: &Point, map: &Vec<Vec<char>>) -> bool {
    let mut valid_move = false;
    if p.x < map.len()-1 {
        match map[p.x+1][p.y] {
            '.' => valid_move = true,
            '#' => valid_move = false,
            'S' => valid_move = true,
            _ => panic!("==> No point found in check_north! <==")
        }
    }
    valid_move
}

fn check_east(p: &Point, map: &Vec<Vec<char>>) -> bool {
    let mut valid_move = false;
    if p.y < map[0].len()-1 {
        match map[p.x][p.y+1] {
            '.' => valid_move = true,
            '#' => valid_move = false,
            'S' => valid_move = true,
            _ => panic!("==> No point found in check_north! <==")
        }
    }
    valid_move
}

fn check_west(p: &Point, map: &Vec<Vec<char>>) -> bool {
    let mut valid_move = false;
    if p.y > 0 {
        match map[p.x][p.y-1] {
            '.' => valid_move = true,
            '#' => valid_move = false,
            'S' => valid_move = true,
            _ => panic!("==> No point found in check_north! <==")
        }
    }
    valid_move
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let contents = lines_from_file(filename);
    contents.iter()
        .map(|s| s.chars().collect())
        .collect()
}