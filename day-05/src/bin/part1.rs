use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 5!");

    let path = "./src/data/input1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: Vec<String>) -> i64 {
    // get the seeds
    let seeds: Vec<i64> = input[0]
        .clone()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    println!("The seeds are: {:?}", seeds);

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
    vec_of_maps.push(map_keys.clone()); // get last map
    // println!("The vec of maps: {:?}", vec_of_maps);

    let s2s_maps = Vec::from(&vec_of_maps[1][1..]);
    let s2f_maps = Vec::from(&vec_of_maps[2][1..]);
    let f2w_maps = Vec::from(&vec_of_maps[3][1..]);
    let w2l_maps = Vec::from(&vec_of_maps[4][1..]);
    let l2t_maps = Vec::from(&vec_of_maps[5][1..]);
    let t2h_maps = Vec::from(&vec_of_maps[6][1..]);
    let h2l_maps = Vec::from(&vec_of_maps[7][1..]);

    let mut locations: Vec<i64> = Vec::new();
    for seed_start in seeds {
        let mut seed = seed_start.clone();
        // Journey of a seed from seed -> location
        println!("======== The seed map ===============");
        //=============================================
        println!("============================");
        // seed to soil
        seed = start_to_destination(seed, &s2s_maps);
        println!("Seed to soil map: {}", seed);
        //=============================================
        println!("============================");
        // soil to fertilizer
        seed = start_to_destination(seed, &s2f_maps);
        println!("Soil to fertilizer map: {}", seed);
        //=============================================
        println!("============================");
        // fertilizer to water
        seed = start_to_destination(seed, &f2w_maps);
        println!("Fertilizer to water map: {}", seed);
        //=============================================
        println!("============================");
        // water to light
        seed = start_to_destination(seed, &w2l_maps);
        println!("Water to light map: {}", seed);
        //=============================================
        println!("============================");
        // light to temperature
        seed = start_to_destination(seed, &l2t_maps);
        println!("Light to temperature map: {}", seed);
        //=============================================
        println!("============================");
        // temperature to humidity
        seed = start_to_destination(seed, &t2h_maps);
        println!("Temperature to humidity map: {}", seed);
        //=============================================
        println!("============================");
        // humidity to location
        seed = start_to_destination(seed, &h2l_maps);
        println!("Humidity to location map: {}", seed);
        //=============================================
        // push location to locations vector
        locations.push(seed);
    }

    locations.iter().min().unwrap().clone() // return
}

fn start_to_destination(mut seed: i64, maps: &Vec<String>) -> i64 {
    // s2s_maps e.g., ["seed-to-soil map:", "50 98 2", "52 50 48", ... n]
    // process s2s_maps
    let nums = map_to_num(maps);
    // loop over lines in the map to modify seed accordingly
    // general formula
    // if seed within boundary then seed = seed - source + destination
    for map in nums {
        if (map[1] <= seed) && (seed < map[1] + map[2]) {
            seed = seed - map[1] + map[0];
            break;
        }
        println!("Inside start to destination: {:?}", seed);
    }
    seed // return
}

fn map_to_num(map_strings: &Vec<String>) -> Vec<Vec<i64>> {
    map_strings.iter()
        .map(|s|
            s.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
            )
        .collect()
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