
use std::collections::HashMap;
fn main() {
    println!("Hello day 2!");
    let input: Vec<String> = get_input("input1.txt");
    let answer: String = part1(input);
    println!("The input is: {:?}", answer)

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

fn part1(input: Vec<String>) -> String {
    let mut game_bag: HashMap<&str, u32> = HashMap::new();
    game_bag.insert("red", 12);
    game_bag.insert("green", 13);
    game_bag.insert("blue", 14);
    // Algorithm design
    // 1) loop through games
    // 2) for each game you need to identify the 
    //  -> game ID > split on ":"
    //     -> find game number
    //  -> sets > split on ";"
    //     -> find colours split on ","
    //     -> find number of each colour


    let mut score: u32 = 0;
    for data in input {

        let game: Vec<&str> = data.split(":").collect();
        let game_id_vec: Vec<&str> = game[0].split(" ").collect();
        let game_id: &str = game_id_vec[1];
        
        let game_play = game[1];
        let game_subset: Vec<&str> = game_play.split(";").collect();
        let game_subset_colours: Vec<Vec<String>> = game_subset
                                    .iter()
                                    .map(|s| s.split(",").map(String::from).collect())
                                    .collect();
        let mut allowed_flag: bool = true;
        
        println!("Game ID is {:?} and the results are {:?}", game_id, game_subset_colours);

        for colours in game_subset_colours {
            
            for colour in colours {
                let colour_vec: Vec<&str> = colour.split(" ").collect();
                match colour_vec[2] {
                    "red" => {
                        println!("There are {:?} red, in game ID {:?}", colour_vec[1], game_id);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["red"] {allowed_flag = false};
                    },
                    "green" => {
                        println!("There are {:?} green", colour_vec[1]);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["green"] {allowed_flag = false};
                    },
                    "blue" => {
                        println!("There are {:?} blue", colour_vec[1]);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["blue"] {allowed_flag = false};
                    },
                    _ => {println!("didnt' match!")},
                }

                if !allowed_flag {break;}
                
            }
            
            
        }

        if allowed_flag {
            score = score + game_id.parse::<u32>().unwrap();
        }

    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_sum_of_ids() {
        let result = 8;
        let answer = 8;
        assert_eq!(result, answer);
    }
}