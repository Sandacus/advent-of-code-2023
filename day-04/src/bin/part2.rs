use std::collections::BTreeMap;

fn main() {
    println!("Hello, day 4!");

    let input = get_input("input1.txt");
    println!("{:?}", input);

    let answer: i32 = part1(input);
    println!("The answer is: {}", answer);
}


fn get_input(file_name: &str) -> Vec<String> {
    // get the input
    let mut file_path: String = std::env::current_dir()
                                    .unwrap()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap();
    file_path = file_path + "/" + file_name;
    std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_matches(player: Vec<&str>, answers: Vec<&str>) -> i32 {
    let mut matches = 0;
    for num in player {
        if answers.contains(&num) {
            matches += 1;
        }
    }
    matches
}


fn part1(data: Vec<String>) -> i32 {
    let score_cards_original = data;
    
    //create map to scorecards
    let mut score_cards: Vec<i32> = vec![1; score_cards_original.len()];

    // index counter for the current score card
    let mut current: usize = 0;

    for card in score_cards_original {

        let card_pieces: Vec<&str> = card.splitn(2, ':').collect();
        let game_id = card_pieces[0].trim();
        let game_pieces: Vec<&str> = card_pieces[1].split('|').collect();
        let answers: Vec<&str> = game_pieces[0].trim().split_whitespace().collect();
        let player: Vec<&str> = game_pieces[1].trim().split_whitespace().collect();

        // add score cards to the next number of scorecards
        // equal to matches * num of copies
        let start = current + 1;
        let stop = current + (get_matches(player, answers) + 1) as usize;
        for i in start..stop {
            score_cards[i] += score_cards[current];
        }

        

        current += 1;
        
    }

    score_cards.iter().sum()
    // 123 // dummy return
    
}