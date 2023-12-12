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
    // get the seeds
    let seeds: Vec<i64> = input[0]
        .clone()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    // get the maps
    let mut vec_of_maps: Vec<Vec<String>> = Vec::new();
    let mut map_keys: Vec<String> = Vec::new();
    for s in input {
        if s == "" {
            vec_of_maps.push(map_keys.clone());
            map_keys = Vec::new();
        } else {
            map_keys.push(s);
        }
    }

    // convert strings to numbers
    for mut map_key in vec_of_maps {
        for i in 1..map_key.len() {
            map_key[i] = map_key[i].split_whitespace().map(|n| n.clone().parse::<i64>()).unwrap().collect();
            println!("The nums are: {:?}", map_key[i]);
        }
    }

    // Journey of a seed from seed -> location
    // seed to soil
    // let seed = seed_to_soil(42, &vec_of_maps);
    // println!("The soil map: \n{:?}", seed);

    // soil to fertilizer
    // map = soil_to_fertilizer(map);

    // fertilizer to water
    // map = fertilizer_to_soil(map);

    // water to light
    // map = water_to_light(map);

    // light to temperature
    // map = light_to_temperature(map);

    // temperature to humidity
    // map = temperature_to_light(map);

    // humidity to location
    // map = humidity_to_location(map);

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

fn seed_to_soil(mut seed: i64, vec_of_maps: &Vec<Vec<String>>) -> i64 {
    // loop over lines in the map to modify seed accordingly


    let l1 = vec![50, 98, 2];
    let mut seed = 10;
    // first line
    if (98 <= seed) && (seed < 98 + 2) {
        seed = seed - 98 + 50;
    }

    42 // dummy return
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(path: &str) -> Vec<String> {
    let path = path;
    let contents = lines_from_file(path);
    // println!("File contents: {:?}", contents);
    contents
}