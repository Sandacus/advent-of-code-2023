use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("Hello, day 7!");
    let path = "./src/data/input1.txt";
    let input = get_input(path);

    let start = Instant::now();
    let ans = part2(input);
    println!("The answer is: {:?}", ans);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn part2(input: Vec<String>) -> i32 {
    // make a vector of the hands
    let mut hands: Vec<String> = input
        .iter()
        .map(|s| s[..5].to_string())
        .collect();
    // make a vector of the bids
    let bids: Vec<String> = input
        .iter()
        .map(|s| s[6..].to_string())
        .collect();
    // create a hashmap of hands and bids
    let mut hands_map = HashMap::new();
    for i in 0..hands.len() {
        hands_map.insert(hands[i].clone(), bids[i].parse::<i32>().unwrap());
    }
    // rank the hands with bubble sort
    for i in 0..hands.len() - 1 {
        let mut swapped = false;
        for j in 0..hands.len() - 1 - i {
            if hands[j] == get_higher_rank(&hands[j], &hands[j + 1]) {
                let dummy = hands[j].clone();
                hands[j] = hands[j + 1].clone();
                hands[j + 1] = dummy;
                swapped = true;
            }
        }
        if !swapped { break; }
    }
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hands_map.get(hand).unwrap() * (i as i32 + 1);
    }

    winnings // return
}

fn get_higher_rank(hand1: &String, hand2: &String) -> String {
    let score1 = get_hand_rank(hand1);
    let score2 = get_hand_rank(hand2);

    if score1 > score2 {
        return hand1.clone();
    } else if score1 < score2 {
        return hand2.clone();
    } else if score1 == score2 {
        return decider(hand1, hand2);
    }

    panic!("Can't find a higher rank in fn get_higher_rank!") //return
}

fn get_hand_rank(hand: &String) -> i32 {
    let mut joker_count = 0;

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

    for card in hand.chars() {
        match card {
            'J' => joker_count += 1,
            '2' => *card_count.entry('2'.to_owned()).or_default() += 1,
            '3' => *card_count.entry('3'.to_owned()).or_default() += 1,
            '4' => *card_count.entry('4'.to_owned()).or_default() += 1,
            '5' => *card_count.entry('5'.to_owned()).or_default() += 1,
            '6' => *card_count.entry('6'.to_owned()).or_default() += 1,
            '7' => *card_count.entry('7'.to_owned()).or_default() += 1,
            '8' => *card_count.entry('8'.to_owned()).or_default() += 1,
            '9' => *card_count.entry('9'.to_owned()).or_default() += 1,
            'T' => *card_count.entry('T'.to_owned()).or_default() += 1,
            'Q' => *card_count.entry('Q'.to_owned()).or_default() += 1,
            'K' => *card_count.entry('K'.to_owned()).or_default() += 1,
            'A' => *card_count.entry('A'.to_owned()).or_default() += 1,
            _ => panic!(),
        }
    }

    // get rid of zeros
    card_count.retain(|_, &mut v| v != 0);

    let cards_count = card_count.values().max();
    let max_cards: i32;
    if cards_count.is_some() {
        max_cards = joker_count + cards_count.unwrap();
    } else {
        max_cards = joker_count;
    }

    let min_cards_count = card_count.values().min();
    let min_cards: i32;
    if cards_count.is_some() {
        min_cards = min_cards_count.unwrap().clone();
    } else {
        min_cards = 0;
    }

    // Now test for the type of hand
    let hand_rank = match max_cards {
        5 => 7,
        4 => 6,
        3 => if min_cards == 2 { 5 } else { 4 },
        2 => if card_count.len() == 3 { 3 } else { 2 },
        1 => 1,
        _ => panic!(),
    };

    hand_rank // return
}

fn decider(hand1: &String, hand2: &String) -> String {
    // compare the two hands
    let cards1: Vec<char> = hand1.chars().collect();
    let cards2: Vec<char> = hand2.chars().collect();
    for i in 0..5 {
        if card_value(cards1[i]) > card_value(cards2[i]) {
            return hand1.clone();
        } else if card_value(cards1[i]) < card_value(cards2[i]) {
            return hand2.clone();
        } else { continue; }
    }
    println!("cards 1: {:?} vs cards 2: {:?}", cards1, cards2);
    panic!("Couldn't decide a higher hand in fn decider!") // return
}

fn card_value(card: char) -> i32 {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 15,
        _ => panic!(),
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| l != "")
        .collect()
}

fn get_input(filename: impl AsRef<Path>) -> Vec<String> {
    let contents = lines_from_file(filename);
    contents
}