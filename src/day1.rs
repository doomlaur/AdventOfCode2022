use crate::utils;

pub fn run(load_test_file: bool) {
    println!("Running day 1...");
    
    let path = utils::open_file(1, load_test_file);
    
    let mut calories_per_elf : Vec<u64> = Vec::new();

    if let Ok(lines) = utils::read_lines(path.as_path()) {
        let mut sum = 0u64;
        
        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                if line.is_empty() {
                    calories_per_elf.push(sum);
                    sum = 0;
                }
                else {
                    let result = line.parse::<u64>().unwrap();
                    sum += result;
                }
            }
        }
    }

    part1(&calories_per_elf);
    part2(&calories_per_elf);
}

fn part1(calories_per_elf : &Vec<u64>) {
    let max_value = calories_per_elf.iter().max();
    match max_value {
        Some(max) => println!("Part 1 - Max value: {}", max),
        None => println!("Part 1 - ERROR: vector is empty"),
    }
}

fn part2(calories_per_elf : &Vec<u64>) {
    // Too bad that there's no std::partial_sort. A crate exists,
    // but I don't need good performance for AoC :)
    let mut sorted_calories_per_elf = calories_per_elf.to_vec();
    sorted_calories_per_elf.sort();
    
    // Go backwards and calculate sum.
    let needed_elements = 3;
    let mut num_considered_elements = 0;
    let mut sum_highest = 0u64;
    
    for calories in sorted_calories_per_elf.iter().rev() {
        if num_considered_elements >= needed_elements {
            break;
        } else {
            sum_highest += calories;
            num_considered_elements += 1;
        }
    }
    
    if num_considered_elements == needed_elements {
        println!("Part 2 - highest {} calories: {}", needed_elements, sum_highest);
    } else { 
        println!("Part 2 - ERROR, calculated {} instead of {} elements", num_considered_elements, needed_elements);
    }
}