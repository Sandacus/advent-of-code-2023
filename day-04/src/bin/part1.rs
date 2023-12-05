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


fn part1(data: Vec<String>) -> i32 {
    let score_cards = data;
    let mut score = 0;
    // get each game
    for card in score_cards {

        let card_pieces: Vec<&str> = card.splitn(2, ':').collect();
        let game_id = card_pieces[0].trim();
        let game_pieces: Vec<&str> = card_pieces[1].split('|').collect();
        let answers: Vec<&str> = game_pieces[0].trim().split_whitespace().collect();
        let player: Vec<&str> = game_pieces[1].trim().split_whitespace().collect();

        let mut matches = 0;
        for num in player {
            if answers.contains(&num) {
                matches += 1;
            }
        }

        if matches > 0 {
            score += 2_i32.pow(matches-1);
        }
        
    }
    score
}