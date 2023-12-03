
use std::collections::HashMap;
fn main() {
    println!("Hello day 2!");
    let input: Vec<String> = get_input("input1.txt");
    let answer: String = part2(input);
    println!("The answer is: {:?}", answer)

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

fn part2(input: Vec<String>) -> String {
    
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
        let mut game_bag: HashMap<&str, u32> = HashMap::new();
    game_bag.insert("red", 0);
    game_bag.insert("green", 0);
    game_bag.insert("blue", 0);

        let game: Vec<&str> = data.split(":").collect();
        let game_id_vec: Vec<&str> = game[0].split(" ").collect();
        let game_id: &str = game_id_vec[1];
        
        let game_play = game[1];
        let game_subset: Vec<&str> = game_play.split(";").collect();
        let game_subset_colours: Vec<Vec<String>> = game_subset
                                    .iter()
                                    .map(|s| s.split(",").map(String::from).collect())
                                    .collect();
        
        println!("Game ID is {:?} and the results are {:?}", game_id, game_subset_colours);

        for colours in game_subset_colours {
            
            for colour in colours {
                let colour_vec: Vec<&str> = colour.split(" ").collect();
                match colour_vec[2] {
                    "red" => {
                        println!("There are {:?} red, in game ID {:?}", colour_vec[1], game_id);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["red"] {
                            game_bag.entry("red").and_modify(|red| *red = colour_vec[1].parse::<u32>().unwrap()).or_insert(100);
                            };
                    },
                    "green" => {
                        println!("There are {:?} green", colour_vec[1]);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["green"] {
                            game_bag.entry("green").and_modify(|green| *green = colour_vec[1].parse::<u32>().unwrap()).or_insert(100);
                            };
                    },
                    "blue" => {
                        println!("There are {:?} blue", colour_vec[1]);
                        if colour_vec[1].parse::<u32>().unwrap() > game_bag["blue"] {
                            game_bag.entry("blue").and_modify(|blue| *blue = colour_vec[1].parse::<u32>().unwrap()).or_insert(100);
                            };
                    },
                    _ => {println!("didnt' match!")},
                }
                
            }
               
        }

        score = score + game_bag.values().product::<u32>();
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