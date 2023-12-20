use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 19!");
    let path = "./src/data/test1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part2(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: Vec<String>) -> i64 {
    // take a look at the input
    println!("{:?}", input);

    let mid = input.iter().position(|s| s == "").unwrap();
    let rules_input = input[..mid].to_vec();
    let parts_input = input[mid + 1..].to_vec();
    println!("rules input: {:?}", rules_input);
    println!("parts input: {:?}", parts_input);

    // create hashmap for rule name and rules
    let mut name: Vec<String> = Vec::new();
    let mut work_map = HashMap::new();
    for rule in &rules_input {
        let (key, val) = rules_parse(rule);
        name.push(key.clone());
        work_map.insert(key, val);
    }
    println!("work map: {:?}", work_map);

    let (mut x, mut m, mut a, mut s): (i64, i64, i64, i64) = (4_000, 4_000, 4_000, 4_000);
    (x, m, a, s) = evaluate_rule(&"in".to_string(), &work_map, (x, m, a, s));

    x * m * a * s // return
}

fn evaluate_rule(name: &String, work_map: &HashMap<String, Vec<Rule>>, (mut x, mut m, mut a, mut s): (i64, i64, i64, i64)) -> (i64, i64, i64, i64) {
    if name == "R" {
        (x, m, a, s) = (-x, -m, -a, -s);
        return (x, m, a, s);
    }
    if name == "A" {
        return (x, m, a, s);
    }
    // loop through rules
    println!("Evaluate rule, name = {}", name);
    let rules = work_map.get(&*name).unwrap();
    for rule in rules {
        match rule.var.as_str() {
            "x>" => x += 4_000 - rule.num.clone(),
            "x<" => x += rule.num.clone(),
            "m>" => m += 4_000 - rule.num.clone(),
            "m<" => m += rule.num.clone(),
            "a>" => a += 4_000 - rule.num.clone(),
            "a<" => a += rule.num.clone(),
            "s>" => s += 4_000 - rule.num.clone(),
            "s<" => s += rule.num.clone(),
            "_" => {},
            _ => panic!("==> No rule match in evaluate_rule fn <=="),
        };

        (x, m, a, s) = evaluate_rule(&rule.point, work_map, (x, m, a, s));
    }

    (x, m, a, s) // dummy return
}

#[derive(Debug)]
struct Rule {
    var: String,
    num: i64,
    point: String,
}

fn rules_parse(wf: &String) -> (String, Vec<Rule>) {
    // parse workflow
    let rules_input = wf.clone();
    // return name of workflow and rules associated with workflow
    // split name from braces { }
    // Find the opening and closing braces
    let (Some(start), Some(end)) = (rules_input.find('{'), rules_input.find('}')) else { panic!("No braces found in rules parse fn") };
    // Extract the substring within the braces
    let name = &wf[0..start];
    // Split the substring by commas and collect into a vector
    let rules: Vec<&str> = wf[start + 1..end].split(',').collect();
    // parse the rules
    // check for '<' or '>'
    let mut rules_vec: Vec<Rule> = Vec::new();
    for rule in &rules {
        let mut var: String = "_".to_string();
        let mut num: i64 = 0;
        let mut point: String = rule.to_string();
        if let (Some(x), Some(y)) = (rule.find('<'), rule.find(':')) {
            var = rule[0..x + 1].to_string();
            num = rule[x + 1..y].parse::<i64>().unwrap();
            point = rule[y + 1..].to_string();
            println!("var: {}, num: {:?}, point: {}", var, num, point);
        } else if let (Some(x), Some(y)) = (rule.find('>'), rule.find(':')) {
            var = rule[0..x + 1].to_string();
            num = rule[x + 1..y].parse::<i64>().unwrap();
            point = rule[y + 1..].to_string();
            println!("var: {}, num: {:?}, point: {}", var, num, point);
        }
        rules_vec.push(Rule { var, num, point });
    }

    (name.to_string(), rules_vec) // return
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let contents = lines_from_file(filename);
    contents
}