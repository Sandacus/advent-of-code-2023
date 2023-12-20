use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 19!");
    let path = "./src/data/input1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part1(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part1(input: Vec<String>) -> i32 {
    // take a look at the input
    println!("{:?}", input);

    let mid = input.iter().position(|s| s == "").unwrap();
    let rules_input = input[..mid].to_vec();
    let parts_input = input[mid+1..].to_vec();
    println!("rules input: {:?}", rules_input);
    println!("parts input: {:?}", parts_input);

    // create hashmap for rule name and rules
    let mut work_map = HashMap::new();
    for rule in &rules_input {
        let (key, val) = rules_parse(rule);
        work_map.insert(key, val);
    }
    println!("work map: {:?}", work_map);

    // create a vector of parts
    let mut parts_vec: Vec<Part> = Vec::new();
    for part in parts_input {
        parts_vec.push(parts_parse(part));
    }
    println!("parts vec: {:?}", parts_vec);

    let mut total = 0;
    // loop through parts vec and apply rules
    for part in parts_vec {
        // with in rule
        let mut wf = evaluate_rule(&part, work_map.get("in").unwrap());
        println!("Initial conversion from 'in' output: {}", wf);

        // run a loop to keep going until result is either A or R
        loop {
            wf = evaluate_rule(&part, work_map.get(wf.as_str()).unwrap());
            if wf == "A" || wf == "R" {
                break;
            }
        }
        println!("Part: {:?}, results in {}", part, wf);
        if wf == "A" {
            total += part.x + part.m + part.a + part.s;
        }
    }

    total // return
}

fn evaluate_rule(part: &Part, rules: &Vec<Rule>) -> String {
    // loop through rules
    let mut wf = "none".to_string();
    for rule in rules {
        match rule.var.as_str() {
            "x>" => if part.x > rule.num { wf = rule.point.clone(); break;},
            "x<" => if part.x < rule.num { wf = rule.point.clone(); break; },
            "m>" => if part.m > rule.num { wf = rule.point.clone(); break; },
            "m<" => if part.m < rule.num { wf = rule.point.clone(); break; },
            "a>" => if part.a > rule.num { wf = rule.point.clone(); break; },
            "a<" => if part.a < rule.num { wf = rule.point.clone(); break; },
            "s>" => if part.s > rule.num { wf = rule.point.clone(); break; },
            "s<" => if part.s < rule.num { wf = rule.point.clone(); break; },
            "_" => wf = rule.point.clone(),
            _ => panic!("==> No rule match in evaluate_rule fn <=="),
        };
    }

    wf // dummy return
}

#[derive(Debug)]
struct Rule {
    var: String,
    num: i32,
    point: String,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
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
        let mut num: i32 = 0;
        let mut point: String = rule.to_string();
        if let (Some(x), Some(y)) = (rule.find('<'), rule.find(':')) {
            var = rule[0..x+1].to_string();
            num = rule[x+1..y].parse::<i32>().unwrap();
            point = rule[y+1..].to_string();
            println!("var: {}, num: {:?}, point: {}", var, num, point);
        } else if let (Some(x), Some(y)) = (rule.find('>'), rule.find(':')) {
            var = rule[0..x+1].to_string();
            num = rule[x+1..y].parse::<i32>().unwrap();
            point = rule[y+1..].to_string();
            println!("var: {}, num: {:?}, point: {}", var, num, point);
        }
        rules_vec.push(Rule {var, num, point});
    }

    (name.to_string(), rules_vec) // return
}

fn parts_parse(wf: String) -> Part {
    // parse parts
    // return vector of parts
    // split name from braces { }
    // Find the opening and closing braces
    let (Some(start), Some(end)) = (wf.find('{'), wf.find('}')) else { panic!("No braces in parts parse fn") };
    // Extract the substring within the braces
    // Split the substring by commas and collect into a vector
    let parts: Vec<&str> = wf[start + 1..end].split(',').collect();
    // parse the rules
    // check for '='
    // get x number
    let Some(i) = parts[0].find('=') else { panic!("No equals sign in 'x' parts parse fn") };
    let x = parts[0][i+1..].parse::<i32>().unwrap();

    // get m number
    let Some(i) = parts[1].find('=') else { panic!("No equals sign in 'm' parts parse fn") };
    let m = parts[1][i+1..].parse::<i32>().unwrap();

    // get a number
    let Some(i) = parts[2].find('=') else { panic!("No equals sign in 'a' parts parse fn") };
    let a = parts[2][i+1..].parse::<i32>().unwrap();

    // get s number
    let Some(i) = parts[3].find('=') else { panic!("No equals sign in 'm' parts parse fn") };
    let s = parts[3][i+1..].parse::<i32>().unwrap();

    println!("x: {}, m: {}, a: {}, s: {}", x, m, a, s);

    Part {x, m, a, s} // return
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