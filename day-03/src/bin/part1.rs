use std::collections::{HashMap, BTreeMap};

fn main() {
    println!("Hello day 3!");

    let (engine_schematic, num_lines, line_width) = get_input("test3.txt");
    println!("number of lines: {},\nline width: {}", num_lines, line_width);
    println!("The engine schematic looks like this => {:?}", engine_schematic);

    let input: (Vec<String>, usize, usize) = (engine_schematic, num_lines, line_width);
    let answer: String = part1(input);
    println!("The answer is: {:?}", answer)
}


fn get_input(file_name: &str) -> (Vec<String>, usize, usize) {

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
    
    let num_lines: usize = input_vec.len();
    
    return (input_vec, line_width, num_lines);
}


fn part1(input: (Vec<String>, usize, usize)) -> String {
    
    /*
    Algorithm design

    Edge location
    .....
    1....
    .....
    __     _    __
    .....|1....|.....
    allowable places behind: index-(line width -1), index-linewidth
    allowable places in front: index+1, index+linewidth, index+linewidth+1

    Non-edge location
    .....
    ..1..
    .....
     ___   _ _   ___
    .....|..1..|.....
    allowable places behind: index-1, index-(linewidth-1), index-linewidth, index-(linewidth+1)
    allowable places in front: index+1, index+(linewidth-1), n+linewidth, n+(linewidth+1)


    1) Create a hashmap of all numbers using number as value and index as the key.
    2) Apply the rules for looking up the characters surrounding the numbers to see if they are allowed.
    3) Sum allowed numbers.

    */


    // let numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let symbols = ['*', '#', '$', '+', '%', '/', '-', '@', '=', '!', '&'];
    let not_symbols = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

    // unpack input
    let engine_schematic: Vec<String> = input.0;
    let mut engine_schematic_chars: Vec<char> = Vec::new();
    let num_lines = input.1;
    let line_width: usize= input.2;

    // create vector of all characters
    for line in engine_schematic {
        for c in line.chars() {
            engine_schematic_chars.push(c);
        }
    }

    // pick out all numbers from engine schematic
    let mut parts: Vec<char> = Vec::new();
    let mut parts_map: BTreeMap<usize, char> = BTreeMap::new();
    let mut idx: usize = 0;
    for c in &engine_schematic_chars {
        if numbers.contains(&c) {
            parts.push(*c);
            parts_map.insert(idx, *c);
        }
        idx += 1;
    }

    // insert dummy idx at the end 
    parts_map.insert(idx+1, 'X');

    println!("The parts map is: {:?}", parts_map);

    // combine numbers with keys that are next to each other 
    // as long as they are on the same "line" i.e., their index / line width (rounded down) is equal
    let mut previous_key: usize = 0;
    // let mut next_key: usize = *parts_map.keys().nth(1).unwrap();
    let mut number_builder: String = String::new();
    let mut idx_builder: Vec<usize> = Vec::new();
    let mut part_numbers: HashMap<String, Vec<usize>> = HashMap::new();
    for (key, val) in parts_map {
        // println!("key {}: value {}", key, val);

        // check to see if we can combine numbers
        if key == previous_key {
            // combine numbers
            number_builder.push(val);
            idx_builder.push(key);
        } else {
            if (key)/line_width == (previous_key)/line_width && (key-1 == previous_key) {
                // combine numbers
                number_builder.push(val);
                idx_builder.push(key);
            } else {
                // add number from number builder as key and index vector as value to part_numbers map
                part_numbers.insert(number_builder.clone(), idx_builder.clone());
    
                // reset number builder and index builder
                number_builder = String::new();
                number_builder.push(val);
                idx_builder = Vec::new();
                idx_builder.push(key);
    
            }
            
            previous_key = key;
        }
    }
        

    println!("The part numbers are: {:?}", part_numbers);


    //=====================================================
    // TEST NUMBERS
    //=====================================================
    // go through parts numbers to find the valid ones
    // use the array for each part number and the equations to find the 
    // indices to check in the character vector

    let mut allowed_part_numbers: Vec<String> = Vec::new();

    for (key, value) in part_numbers {
        for index in value {
            
            // check if edge location
            if index % line_width == 0 || (index + 1) % line_width == 0 {
                
                if index < line_width { // on first line
                    println!("this is edge location on first line with index {} and character {}",(index), engine_schematic_chars[index]);
                    if index%line_width == 0 {
                        // then at start
                        if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    } else {
                        // at end
                        if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    }
                    //forwards
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                } else if index / line_width + 1 == num_lines { // on last line
                    println!("this is edge location on last line with index {} and character {}",(index), engine_schematic_chars[index]);
                    if index%line_width == 0 {
                        // then at start
                        if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    } else {
                        // at end
                        if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    }
                    // backwards
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                } else { // middle line
                    println!("this is edge location in a middle line with index {} and character {}",(index), engine_schematic_chars[index]);
                    // test for start or end
                    if index%line_width == 0 {
                        // then at start
                        if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    } else {
                        // at end
                        if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                            allowed_part_numbers.push(key);
                            break;
                        }
                    }
                    // forwards
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    // backwards
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                }

            } else { // non-edge location
                if index < line_width { // on first line
                    println!("this is mid location on first line with index {} and character {}",(index), engine_schematic_chars[index]);
                    //test backwards
                    if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    //test forwards
                    if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                } else if index / line_width + 1 == num_lines { // on last line
                    println!("this is mid location on last line with index {} and character {}",(index), engine_schematic_chars[index]);
                    //backwards
                    if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    //forwards
                    if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                } else { // middle line
                    println!("this is mid location in a middle line with index {} and character {}",(index), engine_schematic_chars[index]);
                    //test forwards
                    if !(not_symbols.contains(&engine_schematic_chars[index+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index+line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    //test backwards
                    if !(not_symbols.contains(&engine_schematic_chars[index-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width-1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }  
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width])) {
                        allowed_part_numbers.push(key);
                        break;
                    } 
                    if !(not_symbols.contains(&engine_schematic_chars[index-line_width+1])) {
                        allowed_part_numbers.push(key);
                        break;
                    }
                }
            }
        }
    }

    println!("The allowed numbers are {:?}", allowed_part_numbers);
    let scores: usize = allowed_part_numbers.iter().map(|s| s.parse::<usize>().unwrap()).sum();

    // let mut scores = 0;
    // for num in allowed_part_numbers {
    //     scores = scores + num.parse::<i32>().unwrap();
    // }
    // equals: 256307
    scores.to_string()
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