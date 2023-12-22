use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;
use num::ToPrimitive;

fn main() {
    println!("Hello, day 21!");
    let path = "./src/data/test1.txt";
    let input = get_input(path);
    let steps: usize = 2;

    let start = Instant::now();
    let ans = part2(steps, input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn part2(steps: usize, input: Vec<Vec<char>>) -> i64 {
    let rows = input.len()-1;
    let cols = input[0].len()-1;

    // find start position, S
    let mut start = Point {x: 0, y: 0};
    for (i, v) in input.iter().enumerate() {
        for (j, ch) in v.iter().enumerate() {
            if ch == &'S' {
                let start_x = i as i32;
                let start_y = j as i32;
                start = Point { x: start_x, y: start_y };
                break;
            }
        }
    }
    println!("Start point: {:?}", start);
    // initialise vector of current positions
    let mut new_points: Vec<Point> = Vec::new();
    let mut plots: Vec<Point> = vec![start];

    // loop through steps
    for step in 0..steps {
        new_points.clear();
        // loop through current locations
        for loc in &plots {
            // for each location check north, south, east and west and collect new valid points
            let (north_valid, north_new_map) = check_north(&loc, &input);
            if north_valid {
                let p_n = get_point(&loc, 'n', north_new_map, rows, cols);
                if !new_points.contains(&p_n) {
                    new_points.push(p_n.clone());
                    println!("in check north: {:?}", p_n);
                }
            }
            let (south_valid, south_new_map) = check_south(&loc, &input);
            if south_valid {
                let p_s = get_point(&loc, 's', south_new_map, rows, cols);
                if !new_points.contains(&p_s) {
                    new_points.push(p_s.clone());
                    println!("in check south: {:?}", p_s);
                }
            }
            // let (east_valid, east_new_map) = check_east(&loc, &input);
            // if east_valid {
            //     let p_e = get_point(&loc, 'e', east_new_map, rows, cols);
            //     if !new_points.contains(&p_e) {
            //         new_points.push(p_e.clone());
            //         println!("in check east: {:?}", p_e);
            //     }
            // }
            // let (west_valid, west_new_map) = check_west(&loc, &input);
            // if west_valid {
            //     let p_w = get_point(&loc, 'w', west_new_map, rows, cols);
            //     if !new_points.contains(&p_w) {
            //         new_points.push(p_w.clone());
            //         println!("in check west: {:?}", p_w);
            //     }
            // }
        }
        println!("New points after {} steps are: {:?}",step+1, new_points);
        plots = new_points.clone();
        println!("The plots after {} steps are: {:?}",step+1, plots);
    }
    println!("The number of plots after {} steps is: {:?}",steps, plots.len());

    plots.len() as i64 // return
}

fn get_point(p: &Point, dir: char, new_map: bool, rows: usize, cols: usize) -> Point {
    if dir == 'n' && !new_map {
        let x_n = p.x.clone() - 1;
        let y_n = p.y.clone();
        return Point{ x: x_n, y: y_n };
    }
    if dir == 'n' && new_map {
        let x_n = rows as i32;
        let y_n = p.y.clone();
        return Point{ x: x_n, y: y_n };
    }
    if dir == 's' && !new_map {
        let x_s = p.x.clone() + 1;
        let y_s = p.y.clone();
        return Point{ x: x_s, y: y_s };
    }
    if dir == 's' && new_map {
        let x_s = 0;
        let y_s = p.y.clone();
        return Point{ x: x_s, y: y_s };
    }
    if dir == 'e' && !new_map {
        let x_e = p.x.clone();
        let y_e = p.y.clone() + 1;
        return Point{ x: x_e, y: y_e };
    }
    if dir == 'e' && new_map {
        let x_e = p.x.clone();
        let y_e = 0;
        return Point{ x: x_e, y: y_e };
    }
    if dir == 'w' && !new_map {
        let x_w = p.x.clone();
        let y_w = p.y.clone() - 1;
        return Point{ x: x_w, y: y_w };
    }
    if dir == 'w' && new_map {
        let x_w = p.x.clone();
        let y_w = cols as i32;
        return Point{ x: x_w, y: y_w };
    }
    p.clone()
}

fn check_north(p: &Point, map: &Vec<Vec<char>>) -> (bool, bool) {
    let length = (map.len()-1) as i32;
    let width = (map[0].len()-1) as i32;
    let mut valid_move = false; // check whether it's a plot '.'
    let mut new_map = false; // check whether leaves map onto new map

    // if its above size of map -> need to cast it to a new map
    // find the remainder to get relative position on newly cast map
    let x_remainder = p.x % (length);
    let y_remainder = p.y % (width);

    // get correct character from map
    let mut map_x;
    if p.x < 0 {
        let x_scale: i32 = length + x_remainder - 1; // -1 for looking north
        map_x = x_scale.abs().to_usize().unwrap();
    } else if p.x % length == 0 {
        map_x = 0; // -1 for looking north
    } else {
        map_x = (x_remainder - 1).to_usize().unwrap(); // -1 for looking north
    }
    let map_y;
    if p.y < 0 {
        let y_scale: i32 = width + y_remainder;
        map_y = y_scale.abs().to_usize().unwrap(); // same relative column
    } else if p.y % width == 0 {
        map_y = 0; // same relative column
    } else {
        map_y = y_remainder.to_usize().unwrap(); // same relative column
    }

    match map[map_x][map_y] {
        '.' => (valid_move, new_map) = (true, true),
        '#' => (valid_move, new_map) = (false, true),
        'S' => (valid_move, new_map) = (true, true),
        _ => panic!("==> No point found in check_north! <==")
    }

    (valid_move, new_map) // return
}

fn check_south(p: &Point, map: &Vec<Vec<char>>) -> (bool, bool) {
    let length = (map.len()-1) as i32;
    let width = (map[0].len()-1) as i32;
    let mut valid_move = false; // check whether it's a plot '.'
    let mut new_map = false; // check whether leaves map onto new map

    // if its above size of map -> need to cast it to a new map
    // find the remainder to get relative position on newly cast map
    let x_remainder = p.x % (length);
    let y_remainder = p.y % (width);

    // get correct character from map
    let mut map_x;
    if p.x < 0 {
        let x_scale: i32 = length + x_remainder + 1; // +1 for looking south
        map_x = x_scale.abs().to_usize().unwrap();
    } else if p.x % length == 0 {
        map_x = 0; // -1 for looking north
    } else {
        map_x = (x_remainder + 1).to_usize().unwrap(); // +1 for looking south
    }
    let map_y;
    if p.y < 0 {
        let y_scale: i32 = width + y_remainder;
        map_y = y_scale.abs().to_usize().unwrap(); // same relative column
    } else if p.y % width == 0 {
        map_y = 0; // same relative column
    } else {
        map_y = y_remainder.to_usize().unwrap(); // same relative column
    }

    match map[map_x][map_y] {
        '.' => (valid_move, new_map) = (true, true),
        '#' => (valid_move, new_map) = (false, true),
        'S' => (valid_move, new_map) = (true, true),
        _ => panic!("==> No point found in check_north! <==")
    }

    (valid_move, new_map) // return
}

// fn check_east(p: &Point, map: &Vec<Vec<char>>) -> (bool, bool) {
//     let mut valid_move = false; // check whether it's a plot '.'
//     let mut new_map = false; // check whether leaves map onto new map
//     if p.y < map[0].len()-1 {
//         match map[p.x][p.y+1] {
//             '.' => valid_move = true,
//             '#' => valid_move = false,
//             'S' => valid_move = true,
//             _ => panic!("==> No point found in check_east! <==")
//         }
//     } else {
//         match map[p.x][0] { // look right onto new side of map
//             '.' => (valid_move, new_map) = (true, true),
//             '#' => (valid_move, new_map) = (false, true),
//             'S' => (valid_move, new_map) = (true, true),
//             _ => panic!("==> No point found in check_east! <==")
//         }
//     }
//     (valid_move, new_map)
// }
//
// fn check_west(p: &Point, map: &Vec<Vec<char>>) -> (bool, bool) {
//     let mut valid_move = false; // check whether it's a plot '.'
//     let mut new_map = false; // check whether leaves map onto new map
//     if p.y > 0 {
//         match map[p.x][p.y-1] {
//             '.' => valid_move = true,
//             '#' => valid_move = false,
//             'S' => valid_move = true,
//             _ => panic!("==> No point found in check_west! <==")
//         }
//     } else {
//         match map[p.x][map[0].len()-1] {
//             '.' => (valid_move, new_map) = (true, true),
//             '#' => (valid_move, new_map) = (false, true),
//             'S' => (valid_move, new_map) = (true, true),
//             _ => panic!("==> No point found in check_west! <==")
//         }
//     }
//     (valid_move, new_map)
// }


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