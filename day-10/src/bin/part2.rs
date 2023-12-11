use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::io::Write;
use std::io::Result;

fn main() {
    println!("Hello, day 10!");

    let path = "./src/data/input1.txt";
    let input: Vec<Vec<char>> = get_input(path);

    let (ans, pipeline) = part2(input.clone());
    println!("The answer is: {:?}", ans);

    visualize_path(pipeline, &input);
}

fn visualize_path(pipeline: Vec<[usize; 2]>, input: &Vec<Vec<char>>) -> Result<()> {
    // visualize the path taken
    let rows = input.len();
    let cols = input[0].len();

    let mut pic: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if pipeline.contains(&[i, j]) {
                pic[i][j] = '*';
            } else if input[i][j] == '.' {
                pic[i][j] = '.';
            } else { pic[i].push(' '); }
        }
    }

    println!("{:?}", pic);

    let mut file = File::create("pic.txt")?;

    for row in pic {
        let line: String = row.into_iter().collect();
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn lookup_position(pos: [usize; 2]) -> char {
    let pipe_network: Vec<Vec<char>> = get_input("./src/data/input1.txt");
    pipe_network[pos[0]][pos[1]]
}

fn part2(input: Vec<Vec<char>>) -> (usize, Vec<[usize; 2]>) {
    println!("Here is the input {:?}", input);
    let rows = input.len();
    let cols = input[0].len();


    // Find start position coordinate
    let start = get_start(input);

    // find first move
    let first_move = get_first_move(start, rows, cols);

    // find the path
    pathfinder(start, first_move)
}

fn get_first_move(start_position: [usize; 2], rows: usize, cols: usize) -> [usize; 2] {
    // input: start position
    // output: position of the first valid move

    // look up and if valid send back
    let (up, valid) = look_up(start_position);
    if valid { return up; }

    // look down
    let (down, valid) = look_down(start_position, rows);
    if valid { return down; }

    // look left
    let (left, valid) = look_left(start_position);
    if valid { return left; }

    // look right
    let (right, valid) = look_right(start_position, cols);
    if valid { return right; }

    panic!("Could find a valid first move from start position: {:?}", start_position);
}

fn pathfinder(start_position: [usize; 2], first_move_position: [usize; 2]) -> (usize, Vec<[usize; 2]>) {
    let mut count = 0usize;

    let mut position: [usize; 2] = first_move_position;
    let mut previous_position: [usize; 2] = start_position;
    let mut symbol = lookup_position(position);

    let mut path: Vec<[usize; 2]> = Vec::new();
    path.push(position);

    // traverse pipes!
    loop {
        println!("*******************Inside the LOOP!**********************");
        count += 1;
        println!("loop count: {}", count);

        path.push(position); // add current position to path vector

        // use symbol to update the position
        match symbol {
            '|' => {
                println!("At |: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_ns(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            '-' => {
                println!("At -: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_ew(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position},
            }
            'L' => {
                println!("At L: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_ne(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            'J' => {
                println!("At J: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_nw(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            '7' => {
                println!("At 7: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_sw(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            'F' => {
                println!("At F: {:?} -> {:?}", previous_position, position);
                (position, previous_position) = move_se(position, previous_position); // update to new position
                symbol = lookup_position(position); // look up symbol corresponding to new position
                println!("Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            '.' => {
                println!("NOT A VALID LOCATION => Symbol: {:?} -> Position: {:?}", symbol, position); // print symbol and position
            }
            'S' => {
                println!("'S' -> BACK AT STARTING POSITION");
                break;
            }
            _ => println!("No match found"),
        }

        if count == 1_000_000 { break; }
    }
    let mut ans = 0;
    if count % 2 == 0 {
        ans = count / 2;
    } else {
        ans = count / 2 + 1;
    }
    (ans, path)
}

// F
fn move_se(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    let temp_pos = current_pos;
    if current_pos[0] < previous_pos[0] { // [0,0] [0,1]
        current_pos[1] += 1;
    }
    if current_pos[1] < previous_pos[1] {
        current_pos[0] += 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos)
}

fn move_sw(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    // 7
    let temp_pos = current_pos;
    if current_pos[0] < previous_pos[0] {
        current_pos[1] -= 1;
    }
    if current_pos[1] > previous_pos[1] {
        current_pos[0] += 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos)
}

fn move_nw(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    // J
    let temp_pos = current_pos;
    if current_pos[0] > previous_pos[0] {
        current_pos[1] -= 1;
    }
    if current_pos[1] > previous_pos[1] {
        current_pos[0] -= 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos)
}

fn move_ne(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    // L
    let temp_pos = current_pos;
    if current_pos[0] > previous_pos[0] {
        current_pos[1] += 1;
    }
    if current_pos[1] < previous_pos[1] {
        current_pos[0] -= 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos)
}

fn move_ew(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    // -
    let temp_pos = current_pos;
    if current_pos[1] < previous_pos[1] {
        current_pos[1] -= 1;
    }
    if current_pos[1] > previous_pos[1] {
        current_pos[1] += 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos)
}

fn move_ns(mut current_pos: [usize; 2], mut previous_pos: [usize; 2]) -> ([usize; 2], [usize; 2]) {
    // |
    let temp_pos = current_pos;
    // update position North-South
    // position ij -> i row : j column
    if current_pos[0] < previous_pos[0] {
        current_pos[0] -= 1;
    }
    if current_pos[0] > previous_pos[0] {
        current_pos[0] += 1;
    }
    previous_pos = temp_pos;
    (current_pos, previous_pos) // return updated current and previous positions
}

fn get_start(input: Vec<Vec<char>>) -> [usize; 2] {
    let mut start: [usize; 2] = [0, 0];
    'outer: for i in 0..input.len() {
        for j in 0..input[0].len() {
            println!("{:?}, {:?}", i, j);
            if input[i][j] == 'S' {
                println!("This is the start position: [{:?}, {:?}]", i, j);
                (start[0], start[1]) = (i, j);
                break 'outer;
            }
        }
    }
    println!("The start is position is: {:?}", start);
    start
}

fn look_right(start_position: [usize; 2], cols: usize) -> ([usize; 2], bool) {
    // get character above start position
    // can look up
    if start_position[1] + 1 > cols {
        return (start_position, false);
    }
    let right = [start_position[0], start_position[1] + 1];
    let symbol = lookup_position(right);

    return match symbol {
        '-' => return (right, true),
        '7' => return (right, true),
        'J' => return (right, true),
        _ => return (start_position, false),
    };
}

fn look_left(start_position: [usize; 2]) -> ([usize; 2], bool) {
    // get character above start position
    // can look up
    if start_position[1] - 1 < 0 {
        return (start_position, false);
    }
    let left = [start_position[0], start_position[1] - 1];
    let symbol = lookup_position(left);

    return match symbol {
        '-' => return (left, true),
        'F' => return (left, true),
        'L' => return (left, true),
        _ => return (start_position, false),
    };
}

fn look_down(start_position: [usize; 2], rows: usize) -> ([usize; 2], bool) {
    // get character above start position
    // can look up
    if start_position[0] + 1 > rows {
        return (start_position, false);
    }
    let down = [start_position[0] + 1, start_position[1]];
    let symbol = lookup_position(down);

    return match symbol {
        '|' => return (down, true),
        'J' => return (down, true),
        'L' => return (down, true),
        _ => return (start_position, false),
    };
}

fn look_up(start_position: [usize; 2]) -> ([usize; 2], bool) {
    // get character above start position
    // can look up
    if start_position[0] - 1 < 0 {
        return (start_position, false);
    }
    let up = [start_position[0] - 1, start_position[1]];
    let symbol = lookup_position(up);

    return match symbol {
        '|' => return (up, true),
        '7' => return (up, true),
        'F' => return (up, true),
        _ => return (start_position, false),
    };
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