use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 24!");

    let path = "./src/data/input.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans); // 25_112, 24_815, 17_905 - too high 8491, 14444 not correct
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: Vec<(Vec<f64>, Vec<f64>)>) -> i64 {
    // equation of a line y = mx + c
    // given, x, y, delta_x, delta_y
    // m = delta_y / delta_x
    // c = y - mx

    // intersection of 2 lines
    // => if m1 == m2 --> parallel & never cross
    // =>         y1 = y2
    // =>   mx1 + c1 = mx2 + c2   solve for x, then solve for y to get intersection

    let length = input.len();
    let mut score: i64 = 0;
    let mut pairs  = 0;

    for i in 0..length {
        for j in i+1..length {
            pairs += 1;

            // line 1 for index = i
            let l1 = get_line_eqn(input[i].clone());
            // line 2 for index = j
            let l2 = get_line_eqn(input[j].clone());
            println!("The lines are: \nl1->{:?}, \nl2->{:?}", l1, l2);

            // check for intersection
            let result: ((f64, f64), bool) = two_line_intersect(&l1 ,&l2);
            println!("The result is: {:?}", result);

            let future = check_intersection_future(&result, &input[i], &input[j], l1, l2);

            // check inside test area
            let bounds = (200000000000000.0, 400000000000000.0);
            // let bounds = (7.0 ,27.0);
            if result.1 && check_point_in_test_area(result.0, bounds) && future {
                score += 1;
            }
        }
    }
    println!("The pairs are: {}", pairs);

    score // return
}

fn check_intersection_future(intersection: &((f64, f64), bool), p1: &(Vec<f64>, Vec<f64>), p2: &(Vec<f64>, Vec<f64>), l1: Line, l2: Line) -> bool {
    if !intersection.1 {
        return false;
    }
    // check future
    let mut p1_future = false;
    let mut p2_future = false;


    // point1 future
    if p1.1[0] < 0.0 && intersection.0.0 < p1.0[0] {
        p1_future = true;
    } else if p1.1[0] > 0.0 && intersection.0.0 > p1.0[0] {
        p1_future = true;
    }
    // point2 future
    if p2.1[1] < 0.0 && intersection.0.1 < p2.0[1] {
        p2_future = true;
    } else if p2.1[1] > 0.0 && intersection.0.1 > p2.0[1] {
        p2_future = true;
    }

    if p1_future && p2_future {
        return true;
    } else {
        return false;
    }
}

fn check_point_in_test_area(p: (f64, f64), bounds: (f64, f64)) -> bool {
    // p -> (x, y), bounds -> (lower, upper)
    let check_x = bounds.0 <= p.0 && p.0 <= bounds.1;
    let check_y = bounds.0 <= p.1 && p.1 <= bounds.1;
    if check_x && check_y {
        return true;
    }
    false
}

fn get_line_eqn(line: (Vec<f64>, Vec<f64>)) -> Line {
    // unpack coordinates and gradients
    let x = line.0[0];
    let y = line.0[1];
    let vx = line.1[0];
    let vy = line.1[1];

    let m = vy / vx;
    let c: f64 = y - (m * x);
    Line { m, c }
}

#[derive(Debug)]
struct Line {
    m: f64,
    c: f64,
}
fn two_line_intersect(l1: &Line, l2: &Line) -> ((f64, f64), bool) {
    // check parallel
    if l1.m == l2.m {
        return ((0.0, 0.0), false); // no intersection
    }
    let x = (l2.c - l1.c) / (l1.m - l2.m);
    let y = l1.m * x + l1.c;

    ((x, y), true) // return intersection and intersection flag
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> Vec<(Vec<f64>, Vec<f64>)> {
    let path = path;
    let contents = lines_from_file(path);
    let mut result: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();

    // want output form Vec<([x,y,x], [vx, vy, vz])>
    // split string on @ and trim
    for line in &contents {
        let parts: Vec<String> = line.split("@").map(|s| s.trim().to_string()).collect();
        // coords
        let coords: Vec<f64> = parts[0].split(",").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        let grads: Vec<f64> = parts[1].split(",").map(|s| s.trim().parse::<f64>().unwrap()).collect();

        result.push((coords.clone(), grads.clone()));
    }
    result
}
