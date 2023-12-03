fn main() {
    println!("Hello day 3!");
    let (num_lines, line_width) = get_input("test1.txt");
    println!("number of lines: {},\nline width: {}", num_lines, line_width);
    let engine_schematic = include_str!("test1.txt");
    println!("{:?}", engine_schematic);
    // let answer: String = part1(input);
    // println!("The input is: {:?}", answer)
}


fn get_input(file_name: &str) -> (usize, usize) {

    // return line width, and number of lines
    

    let mut file_path: String = std::env::current_dir()
                                    .unwrap()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap();

    file_path = file_path + "/" + file_name;

    let input_vec: Vec<String> = std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let line_width: usize = input_vec[0].len();
    
    let num_lines: usize= input_vec.len();
    
    return (line_width, num_lines);
}


fn part1(input: Vec<String>) -> String {
    
    // Algorithm design
    /*

    ...
    .1.
    ...
    ___ _ _ ___
    ...|.1.|...
    allowable places behind: n-1, n-2, n-3, n-4
    allowable places in front: n+1, n+2, n+3, n+4

    ...
    1..
    ...
    __   _  __
    ...|1..|...
    allowable places behind: n-2, n-3
    allowable places in front: n+1, n+3, n+4

    1..
    ...
    ...
     _  __
    1..|...|...

    .1.
    ...
    ...
    _ _ ___  
    .1.|...|...
    
    .....
    ..1..
    .....
     ___   _ _   ___
    .....|..1..|.....
    allowable places behind: n-1, n-4, n-5, n-6
    allowable places in front: n+1, n+4, n+5, n+6

    .....
    .1...
    .....
    ___   _ _   ___
    .....|.1...|.....
    allowable places behind: n-1, n-4, n-5, n-6
    allowable places in front: n+1, n+4, n+5, n+6

    .....
    1....
    .....
    __     _    __
    .....|1....|.....
    allowable places behind: n-4, n-5
    allowable places in front: n+1, n+5, n+6

    1) Combine all lines to make 1 long string with total = width x line numbers
    2) Use the width to know whether character needs an edge rule or general rule
    3) 

     */

    
    for row in input {
        println!("{:?}", row);
    }

    
    "1234".to_string()
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