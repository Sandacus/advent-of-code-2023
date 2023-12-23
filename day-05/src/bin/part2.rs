use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};
use indicatif::ProgressIterator;

fn main() {
    println!("Hello, day 5!");

    let path = "./src/data/input1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part2(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: Vec<String>) -> i64 {
    // get the seeds
    println!("Get the seeds!");
    let seeds = get_seeds(input[0].clone());

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

    // loop through seed pairs
    let mut loc_vec = Vec::new();
    for (idx, pair) in seeds.iter().enumerate() {
        println!("Start seed pair {}/{}", idx+1, seeds.len());
        let start = Instant::now();
        let mut seed_vec = Vec::new();
        for x in 0..pair.1 {
            seed_vec.push(pair.0 + x);
        }

        // Parallel processing of the vector
        let mut locations = seed_vec;

        println!("Start seed:soil map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &s2s_maps))
            .collect::<Vec<i64>>();

        println!("Start soil:fertilizer map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &s2f_maps))
            .collect::<Vec<i64>>();

        println!("Start fertilizer:water map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &f2w_maps))
            .collect::<Vec<i64>>();

        println!("Start water:light map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &w2l_maps))
            .collect::<Vec<i64>>();

        println!("Start light:temperature map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &l2t_maps))
            .collect::<Vec<i64>>();

        println!("Start temperature:humidity map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &t2h_maps))
            .collect::<Vec<i64>>();

        println!("Start humidity:location map");
        locations = locations.iter()
            .progress()
            .map(|&x| start_to_destination(x, &h2l_maps))
            .collect::<Vec<i64>>();

        loc_vec.push(locations.iter().min().unwrap().clone());

        let duration = start.elapsed();
        println!("Time elapsed is: {:?}", duration);
    }

    loc_vec.iter().min().unwrap().clone() // return
}

fn get_seeds(seed_map: String) -> Vec<(i64, i64)> {
    // extract numbers from seeds
    let seed_key: Vec<i64> = seed_map
        .clone()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    println!("The seeds are: {:?}", seed_key);
    // build seeds vector from number pairs
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    for i in 0..seed_key.len() {
        if i % 2 == 0 {
                seeds.push((seed_key[i], seed_key[i + 1]));
        }
    }
    seeds
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