use crate::utils;

pub fn run(load_test_file: bool) {
    println!("Running day 4...");

    let path = utils::open_file(4, load_test_file);

    if let Ok(lines) = utils::read_lines(path.as_path()) {
        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                // TODO
            }
        }
    }

    //part1(&calories_per_elf);
    //part2(&calories_per_elf);
}

/*fn part1(calories_per_elf : &Vec<u64>) {
    
}

fn part2(calories_per_elf : &Vec<u64>) {
    
}*/