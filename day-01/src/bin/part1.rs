use std::fs::read_to_string;
fn main() {
    println!("The final answer is: {}", part1("input1.txt"));
}

fn part1(data: &str) -> String {
    let inputs = get_input(data);
    println!("The test case is: {:?}", inputs);

    let valid_chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    
    let mut answer: u32 = 0;

    // loop through inputs in input
    for input in inputs {
        println!("{}", input);

        let mut first_digit: String = "xx".to_string();
        let mut last_digit: String = "yy".to_string();

        // get the first_digit
        for letter in input.chars() {
            if valid_chars.iter().any(|&i| i==letter) {
                first_digit = letter.to_string();
                println!("First digit is: {first_digit}");
                break;
            }
        }

        // get the last digit
        for letter in input.chars().rev() {
            if valid_chars.iter().any(|&i| i==letter) {
                // last_digit = letter.to_digit(10).unwrap();
                last_digit = letter.to_string();
                println!("Last digit is: {last_digit}");
                break;
            }
        }

        let number: u32 = (first_digit + &last_digit).parse().unwrap();
        println!("The number is: {}", number);
        
        answer = answer + number;
        println!("The running total is: {}", answer)
    } 
    
    answer.to_string()
}

fn get_input(file_path: &str) -> Vec<String> {
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
        let result: String = part1("day1example.txt");
        let answer: String = "142".to_string();
        assert_eq!(result, answer);
    }
}