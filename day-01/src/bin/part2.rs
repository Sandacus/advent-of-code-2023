use std::fs::read_to_string;
fn main() {

    let inputs = get_input("test2.txt");
    println!("The test case is: {:?}", inputs);

    println!("The final answer is: {}", part2(inputs));
}

fn part2(inputs: Vec<String>) -> String {

    let valid_strings: Vec<String> = vec!["0".to_string(),
                                        "1".to_string(), "one".to_string(),
                                        "2".to_string(), "two".to_string(),
                                        "3".to_string(), "three".to_string(),
                                        "4".to_string(), "four".to_string(),
                                        "5".to_string(), "five".to_string(),
                                        "6".to_string(), "six".to_string(),
                                        "7".to_string(), "seven".to_string(),
                                        "8".to_string(), "eight".to_string(),
                                        "9".to_string(), "nine".to_string()];
    
    let mut answer: u32 = 0;

    // loop through inputs in input
    for input in inputs {
        println!("{}", input);

        let mut first_digit: String = "xx".to_string();
        let mut last_digit: String = "yy".to_string();
        let mut first_word: String = "".to_string();
        let mut found_first_word: bool = false;
        let mut found_last_word: bool = false;

        
        // get the first_digit
        for letter in input.split("") {

            if valid_strings.iter().any(|i| i == letter) {
                first_digit = letter.to_string();
                println!("The first number is: {}", first_digit);
                break;
            }

            first_word = first_word + letter;

            for i in valid_strings.iter() {
                if first_word.contains(i) {
                    first_digit = i.to_string();
                    println!("The first number is: {}", first_digit);
                    match i.as_str() {
                        "one" => {first_digit = "1".to_string()},
                        "two" => {first_digit = "2".to_string()},
                        "three" => {first_digit = "3".to_string()},
                        "four" => {first_digit = "4".to_string()},
                        "five" => {first_digit = "5".to_string()},
                        "six" => {first_digit = "6".to_string()},
                        "seven" => {first_digit = "7".to_string()},
                        "eight" => {first_digit = "8".to_string()},
                        "nine" => {first_digit = "9".to_string()},
                        _ => println!("No match found!"),
                    }
                    found_first_word = true;
                    break;
                }
            }
            
            if found_first_word {break;}

        }

        // get the last digit
        let reversed_input: String = input.chars().rev().collect();
        let mut reversed_last_word = String::new();
        for letter in reversed_input.split("") {

            if valid_strings.iter().any(|i| i == letter) {
                last_digit = letter.to_string();
                println!("The last number is: {}", last_digit);
                break;
            }

            reversed_last_word = reversed_last_word + letter;

            let last_word: String = reversed_last_word.chars().rev().collect();

            for i in valid_strings.iter() {
                if last_word.contains(i) {
                    last_digit = i.to_string();
                    println!("The last number is: {}", last_digit);
                    match i.as_str() {
                        "one" => {last_digit = "1".to_string()},
                        "two" => {last_digit = "2".to_string()},
                        "three" => {last_digit = "3".to_string()},
                        "four" => {last_digit = "4".to_string()},
                        "five" => {last_digit = "5".to_string()},
                        "six" => {last_digit = "6".to_string()},
                        "seven" => {last_digit = "7".to_string()},
                        "eight" => {last_digit = "8".to_string()},
                        "nine" => {last_digit = "9".to_string()},
                        _ => println!("No match found!"),
                    }
                    found_last_word = true;
                    break;
                }
            }
            
            if found_last_word {break;}
            
        }
        

        let number: u32 = (first_digit + &last_digit).parse().unwrap();
        println!("The number is: {}", number);
        
        answer = answer + number;
        println!("The running total is: {}", answer)
    } 
    
    answer.to_string()
}

fn get_input(file_name: &str) -> Vec<String> {
    let mut file_path: String = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    file_path = file_path + "/" + file_name;
    read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: String = part2(vec!["two1nine".to_string(),
                                                "eightwothree".to_string(),
                                                "abcone2threexyz".to_string(),
                                                "xtwone3four".to_string(),
                                                "4nineeightseven2".to_string(),
                                                "zoneight234".to_string(),
                                                "7pqrstsixteen".to_string()]);
        let answer: String = "281".to_string();
        assert_eq!(result, answer);
    }
}

