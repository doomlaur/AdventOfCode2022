use crate::utils;
use std::collections::HashSet;
use std::path::{PathBuf};

pub fn run(load_test_file: bool) {
    println!("Running day 3...");

    let path = utils::open_file(3, load_test_file);
    
    part1(&path);
    part2(&path);
}

fn part1(path : &PathBuf) {
    if let Ok(lines) = utils::read_lines(path.as_path()) {
        let mut priorities_sum : u64 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                if line.len() > 0 && line.len() % 2 == 0 {
                    let first_half = &line[0 .. line.len() / 2];
                    let second_half = &line[line.len() / 2 ..]; // until the end

                    let set_first_half : HashSet<char> = first_half.chars().collect();
                    let mut common_item : char = '\0';

                    for item in second_half.chars() {
                        if set_first_half.contains(&item) {
                            common_item = item;
                            break;
                        }
                    }

                    if common_item != '\0' {
                        priorities_sum += get_priority_for_item(common_item);
                    } else {
                        panic!("ERROR! Wrong item!")
                    }
                } else {
                    panic!("ERROR! Unexpected number of characters in line.")
                }
            }
        }

        println!("Part 1 - priorities sum: {}", priorities_sum);
    }
}

fn part2(path : &PathBuf) {
    if let Ok(lines) = utils::read_lines(path.as_path()) {
        let mut priorities_sum : u64 = 0;
        let mut current_line_of_group = 0;

        let mut set_first_line : HashSet<char> = HashSet::new();
        let mut set_second_line : HashSet<char> = HashSet::new();
        
        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                current_line_of_group += 1;
                
                if current_line_of_group == 1 {
                    set_first_line = line.chars().collect();
                } else if current_line_of_group == 2 {
                    set_second_line = line.chars().collect();
                } else if current_line_of_group == 3 {
                    let mut common_item : char = '\0';
                    
                    for item in line.chars() {
                        if set_first_line.contains(&item) && set_second_line.contains(&item) {
                            common_item = item;
                            break;
                        }
                    }

                    if common_item != '\0' {
                        priorities_sum += get_priority_for_item(common_item);
                    } else {
                        panic!("ERROR! Wrong item!")
                    }
                    
                    current_line_of_group = 0;
                } else {
                    panic!("ERROR! A group cannot contain so many lines!")
                }
            }
        }

        println!("Part 2 - priorities sum: {}", priorities_sum);
    }
}

fn get_priority_for_item(item: char) -> u64 {
    match item {
        'a'..='z' => 1 + item as u64 - 'a' as u64,
        'A'..='Z' => 27 + item as u64 - 'A' as u64,
        _ => panic!("ERROR! Wrong character!")
    }
}