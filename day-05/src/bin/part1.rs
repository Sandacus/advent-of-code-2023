use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Hello, day 5!");

    let path = "./src/data/test1.txt";
    let input = get_input(path);
    // println!("{:?}", input);

    // let nums: Vec<_> = input
    //     .into_iter()
    //     .filter_map(|s| s.replace(" ", "").parse::<i64>().ok())
    //     .collect();


    let ans = part1(input);
    println!("The answer is: {:?}", ans);
}

fn part1(input: Vec<String>) -> i64 {
    println!("{:?}", input);

    // get the seeds
    let seeds: Vec<i64> = input[0]
        .clone()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    // start map
    let mut map: Vec<i64> = Vec::new();

    // seed to soil
    map = seed_to_soil(map);

    // soil to fertilizer
    map = soil_to_fertilizer(map);

    // fertilizer to water
    map = fertilizer_to_soil(map);

    // water to light
    map = water_to_light(map);

    // light to temperature
    map = light_to_temperature(map);

    // temperature to humidity
    map = temperature_to_light(map);

    // humidity to location
    map = humidity_to_location(map);

    println!("the map is: {:?}", map);
    println!("the seeds are: {:?}", seeds);
    42
}

fn humidity_to_location(map: Vec<i64>) -> Vec<i64> {
    map
}

fn temperature_to_light(map: Vec<i64>) -> Vec<i64> {
    map
}

fn light_to_temperature(map: Vec<i64>) -> Vec<i64> {
    map
}

fn water_to_light(map: Vec<i64>) -> Vec<i64> {
    map
}

fn fertilizer_to_soil(map: Vec<i64>) -> Vec<i64> {
    map
}

fn soil_to_fertilizer(map: Vec<i64>) -> Vec<i64> {
    map
}

fn seed_to_soil(map: Vec<i64>) -> Vec<i64> {
    let s2s_map = "50 98 2";

    map
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