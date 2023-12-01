fn main() {
    let input: &str = include_str!("input1.txt");
    let output: String = part1(input);
    dbg!(input);
}

fn part1(_input: &str) -> String {
    "1234".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result: String = part1("");
        let answer: String = "1234".to_string();
        assert_eq!(result, answer);
    }
}