use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, day 7!");

    let input = get_input();
    let nums: Vec<_> = input
        .into_iter()
        .filter_map(|s| s.replace(" ", "").parse::<i64>().ok())
        .collect();
    println!("{:?}", nums);
    part1();

    get_hand_type();
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input() -> Vec<String> {
    let path = "./src/bin/test1.txt";
    let contents = lines_from_file(path);
    println!("File contents: {:?}", contents);
    contents
}


fn part1() {
    // TODO
    // get the input
    let input = get_input();

    // make a vector of the hands
    let hands: Vec<String> = input
        .iter()
        .map(|s| s[..5].to_string())
        .collect();

    // make a vector of the bids
    let bids: Vec<String> = input
        .iter()
        .map(|s| s[6..].to_string())
        .collect();

    // make a vector of the hand type e.g., 3 of a kind

    println!("{:?}\n{:?}", hands, bids);

}


fn get_hand_type() {
    let mut card_count = HashMap::new();
    card_count.insert('2', 0);
    card_count.insert('3', 0);
    card_count.insert('4', 0);
    card_count.insert('5', 0);
    card_count.insert('6', 0);
    card_count.insert('7', 0);
    card_count.insert('8', 0);
    card_count.insert('9', 0);
    card_count.insert('T', 0);
    card_count.insert('Q', 0);
    card_count.insert('K', 0);
    card_count.insert('A', 0);

    let test_hand = "32T3K";

    for card in test_hand.chars() {
        match card {
            '2' => *card_count.entry('2'.to_owned()).or_default() += 1,
            '3' => *card_count.entry('3'.to_owned()).or_default() += 1,
            '4' => *card_count.entry('4'.to_owned()).or_default() += 1,
            '5' => *card_count.entry('5'.to_owned()).or_default() += 1,
            '6' => *card_count.entry('6'.to_owned()).or_default() += 1,
            '7' => *card_count.entry('7'.to_owned()).or_default() += 1,
            '8' => *card_count.entry('8'.to_owned()).or_default() += 1,
            '9' => *card_count.entry('9'.to_owned()).or_default() += 1,
            'T' => *card_count.entry('T'.to_owned()).or_default() += 1,
            'J' => *card_count.entry('J'.to_owned()).or_default() += 1,
            'Q' => *card_count.entry('Q'.to_owned()).or_default() += 1,
            'K' => *card_count.entry('K'.to_owned()).or_default() += 1,
            'A' => *card_count.entry('A'.to_owned()).or_default() += 1,
             _  => panic!(),
        }
    }

    // Now test for the type of hand
    // 5 of a kind
    // 4 of a kind
    // full house
    // 3 of a kind
    // 2 pair
    // high card

    // find number of same cards
    let num_same_cards = Some(card_count.values().max());

    println!("{:?}", card_count);

}