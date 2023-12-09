use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, day 8!");

    let input = get_input("./src/data/input1.txt");

    println!(
        "The number of steps required is: {:?}",
        part2(input.clone())
    );
    let step_count = part2(input.clone());

    println!("The lcm is: {:?}", find_lcm(step_count));
}

fn find_lcm(step_counts: Vec<i64>) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        let mut max = a;
        let mut min = b;
        if min > max {
            (max, min) = (min, max);
        }
        if b == 0 {
            a
        } else {
            gcd(min, max % min)
        }
    }
    fn lcm(a: i64, b: i64) -> i64 {
        (a * b) / gcd(a, b)
    }
    // lcm(14429, 20570)
    let mut ans = 1;
    // let steps: Vec<i64> = vec![18727, 24253, 14429, 20569, 22411, 16271];
    for num in step_counts {
        println!("{:?}, {:?}", ans, num);
        ans = lcm(num, ans);
    }
    // 2459591974694542268458020
    // 78840118559328560351424
    ans
}

fn part2(input: Vec<String>) -> Vec<i64> {
    // split input into directions and nodes
    let (mut directions, nodes) = (input[0].chars().collect::<Vec<char>>().clone(), &input[1..]);

    let node_map = get_node_map(nodes);

    // get all node keys that end with A

    let mut start_locations: Vec<String> = node_map
        .keys()
        .filter(|&key| key.ends_with('A'))
        .map(|s| s.to_string())
        .collect();

    // let mut location_steps: Vec<u64> = (0..start_locations.len()).collect::<Vec<u64>>();

    let mut location_steps: Vec<i64> = vec![0; start_locations.len()];

    // println!("The directions are {:?}", directions);
    // println!("The node map is:\n{:?}", node_map);

    // let mut location = String::from("AAA");

    for (i, location) in start_locations.iter().enumerate() {
        // find the steps to get to node that ends in Z
        let dirs = directions.clone();
        // let mut loc = location.clone();
        let steps = get_steps(dirs, &node_map, location.to_string());
        location_steps[i] += steps.clone();
    }
    println!("Locations steps: {:?}", location_steps);
    location_steps
}

fn get_steps(
    mut directions: Vec<char>,
    node_map: &HashMap<&str, Vec<&str>>,
    mut location: String,
) -> i64 {
    let mut counter: i64 = 0;
    while counter < 1_000_000_000 {
        location = get_location(&directions, &node_map, &location);
        directions = update_directions(directions);
        counter += 1;
        // println!("Count: {:?}", counter);
        if location.ends_with("Z") {
            break;
        };
    }
    counter
}

fn get_location<'a>(
    directions: &'a [char],
    node_map: &'a HashMap<&'a str, Vec<&'a str>>,
    mut location: &String,
) -> String {
    let mut loc = location.as_str();
    match directions[0] {
        'L' => loc = node_map.get(loc).unwrap()[0],
        'R' => loc = node_map.get(loc).unwrap()[1],
        _ => println!("Match not found!"),
    }
    loc.to_string().clone()
}

fn update_directions(mut directions: Vec<char>) -> Vec<char> {
    directions.push(directions[0]); // last used direction to back
    directions.remove(0); // remove last used direction to front
    directions
}

fn get_node_map(nodes: &[String]) -> HashMap<&str, Vec<&str>> {
    // define a hashmap for the nodes and parse the nodes vector and insert them
    let mut node_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for node in nodes {
        let node_vec: Vec<&str> = node.split(" = ").collect();
        let node_key = node_vec[0];
        let node_values: Vec<&str> = node_vec[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .collect();
        // create a node map
        node_map.insert(node_key, node_values);
    }
    node_map
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(path: &str) -> Vec<String> {
    let path = path;
    let contents = lines_from_file(path);
    // println!("File contents: {:?}", contents);
    contents
}
