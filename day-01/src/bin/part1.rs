use std::fs::read_to_string;
fn main() {
    
    let inputs = get_input("test1.txt");
    println!("The test case is: {:?}", inputs);

    println!("The final answer is: {}", part1(inputs));
}

fn part1(inputs: Vec<String>) -> String {
    
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
        let result: String = part1(vec!["1abc2".to_string(),
                                                "pqr3stu8vwx".to_string(),
                                                "a1b2c3d4e5f".to_string(),
                                                "treb7uchet".to_string()]);
        let answer: String = "142".to_string();
        assert_eq!(result, answer);
    }
}