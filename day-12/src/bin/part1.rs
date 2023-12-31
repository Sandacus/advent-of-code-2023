use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("Hello, day 12!");

    let path = "./src/data/test1.txt";
    let input: Vec<Vec<String>> = get_input(path);
    println!("The input is: {:?}", input);

    let ans = part1(input.clone());
    println!("The answer is: {:?}", ans);
}

fn fib (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
    cache.get(&arg).map(|entry| entry.clone()).unwrap_or_else(|| {
        let result = match arg {
            0 => 0,
            1 => 1,
            n => fib(cache, n - 1) + fib(cache, n - 2),
        };
        cache.insert(arg, result.clone());
        result
    })
}

fn part1(input: Vec<Vec<String>>) -> i64 {

    let x = 10;
    println!("Fib of {} is {}", x, fib(&mut HashMap::new(), x));


    find_combinations();

    42 // dummy return
}

fn find_combinations() -> i64 {
    // data in form ["?.#", "1, 2"]
    let s: Vec<char> = "???.###".chars().collect();
    let n = [1, 1, 3];

    let b: u32 = s.iter().map(|c| if *c == '?' { 1 } else { 0 }).sum();
    let combos: i64 = 2_i64.pow(b);
    println!("The number of ?'s is: {} and total combos is {}", b, combos);

    for i in 0..b {

    }

    42 // dummy return
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(path: &str) -> Vec<Vec<String>> {
    let contents = lines_from_file(path);
    // for mut line in contents {
    //     let _new_line: Vec<&str> = line.as_str().split_whitespace().clone().collect();
    // }
    contents.iter()
        .map(|s| s.split_whitespace().map(str::to_string).collect())
        .collect()
}