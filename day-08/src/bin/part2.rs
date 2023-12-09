use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, day 8!");

    let input = get_input("./src/data/input1.txt");

    println!("The number of steps required is: {:?}", part1(input));
}

fn part1(input: Vec<String>) -> i32 {
    // split input into directions and nodes
    let (mut directions, nodes) = (input[0].chars().collect::<Vec<char>>().clone(), &input[1..]);

    let node_map = get_node_map(nodes);

    // get all node keys that end with A

    let mut start_locations: Vec<String> = node_map
        .keys()
        .filter(|&key| key.ends_with('A'))
        .map(|s| s.to_string())
        .collect();

    println!("The directions are {:?}", directions);
    println!("The node map is:\n{:?}", node_map);

    let mut location = String::from("AAA");
    let mut counter = 0;

    let mut end_reached: bool = false;
    while !end_reached && counter < 1_000_000_000 {
        // iterate through locations
        for i in 0..start_locations.len() {
            let loc = start_locations[i].clone();
            start_locations[i] = get_location(&directions, &node_map, loc);
        }
        directions = update_directions(directions);
        counter += 1;
        println!("Count: {:?}", counter);
        end_reached = start_locations.iter().all(|s| s.ends_with("Z"));
    }
    counter
}

fn get_location<'a>(
    directions: &'a [char],
    node_map: &'a HashMap<&'a str, Vec<&'a str>>,
    mut location: String,
) -> String {
    match directions[0] {
        'L' => location = String::from(node_map.get(location.as_str()).unwrap()[0]),
        'R' => location = String::from(node_map.get(location.as_str()).unwrap()[1]),
        _ => println!("Match not found!"),
    }
    location
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
    println!("File contents: {:?}", contents);
    contents
}
