use crate::utils;

enum Shape {
    Rock,
    Paper,
    Scissors
}

enum GameOutcome {
    Won,
    Draw,
    Lost
}

pub fn run(load_test_file: bool) {
    println!("Running day 2...");

    let path = utils::open_file(2, load_test_file);

    if let Ok(lines) = utils::read_lines(path.as_path()) {
        let mut total_score_part_1 : u64 = 0;
        let mut total_score_part_2 : u64 = 0;
        
        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                let split = line.split(" ");
                let vec : Vec<&str> = split.collect();
                
                if vec.len() == 2 {
                    let opponent_shape_opt = get_opponent_shape_for_string(vec[0]);
                    
                    // Part 1
                    let my_shape_opt = get_my_shape_for_string(vec[1]);
                    
                    match (&opponent_shape_opt, my_shape_opt) {
                        (Some(opponent_shape), Some(my_shape)) => {
                            let my_shape_points = get_points_for_shape(&my_shape);
                            
                            let game_outcome_score_part_1 = get_points_for_game_outcome(&get_my_game_outcome_for_shapes(&opponent_shape, &my_shape));
                            total_score_part_1 += my_shape_points + game_outcome_score_part_1;
                        }
                        _ => panic!("ERROR! Wrong shapes!")
                    }
                    
                    // Part 2
                    let game_outcome_opt = get_game_outcome_for_string(vec[1]);
                    
                    match(opponent_shape_opt, game_outcome_opt) {
                        (Some(opponent_shape), Some(game_outcome)) => {
                            let my_shape = get_my_shape_for_game_outcome(&opponent_shape, &game_outcome);
                            let my_shape_points = get_points_for_shape(&my_shape);
                            
                            let game_outcome_score_part_2 = get_points_for_game_outcome(&get_my_game_outcome_for_shapes(&opponent_shape, &my_shape));
                            total_score_part_2 += my_shape_points + game_outcome_score_part_2;
                        }
                        _ => panic!("ERROR! Wrong shape and/or game outcome!")
                    }
                } else {
                    panic!("ERROR! Wrong number of columns in line!");
                }
            }
        }
        
        println!("Total score part 1: {}", total_score_part_1);
        println!("Total score part 2: {}", total_score_part_2);
    }
}

fn get_opponent_shape_for_string(shape: &str) -> Option<Shape> {
    match shape {
        "A" => Some(Shape::Rock),
        "B" => Some(Shape::Paper),
        "C" => Some(Shape::Scissors),
        _ => None
    }
}

fn get_my_shape_for_string(shape: &str) -> Option<Shape> {
    match shape {
        "X" => Some(Shape::Rock),
        "Y" => Some(Shape::Paper),
        "Z" => Some(Shape::Scissors),
        _ => None
    }
}

fn get_game_outcome_for_string(game_outcome: &str) -> Option<GameOutcome> {
    match game_outcome {
        "X" => Some(GameOutcome::Lost),
        "Y" => Some(GameOutcome::Draw),
        "Z" => Some(GameOutcome::Won),
        _ => None
    }
}

fn get_points_for_shape(shape: &Shape) -> u64 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    }
}

fn get_points_for_game_outcome(outcome: &GameOutcome) -> u64 {
    match outcome {
        GameOutcome::Lost => 0,
        GameOutcome::Draw => 3,
        GameOutcome::Won => 6
    }
}

// Part 1
fn get_my_game_outcome_for_shapes(opponent_shape: &Shape, my_shape: &Shape) -> GameOutcome {
    match (opponent_shape, my_shape) {
        (Shape::Rock, Shape::Rock) => GameOutcome::Draw,
        (Shape::Paper, Shape::Paper) => GameOutcome::Draw,
        (Shape::Scissors, Shape::Scissors) => GameOutcome::Draw,
        (Shape::Scissors, Shape::Rock) => GameOutcome::Won,
        (Shape::Paper, Shape::Scissors) => GameOutcome::Won,
        (Shape::Rock, Shape::Paper) => GameOutcome::Won,
        (Shape::Rock, Shape::Scissors) => GameOutcome::Lost,
        (Shape::Scissors, Shape::Paper) => GameOutcome::Lost,
        (Shape::Paper, Shape::Rock) => GameOutcome::Lost
    }
}

// Part 2
fn get_my_shape_for_game_outcome(opponent_shape: &Shape, game_outcome: &GameOutcome) -> Shape {
    match (opponent_shape, game_outcome) {
        (Shape::Rock, GameOutcome::Draw) => Shape::Rock,
        (Shape::Paper, GameOutcome::Draw) => Shape::Paper,
        (Shape::Scissors, GameOutcome::Draw) => Shape::Scissors,
        (Shape::Scissors, GameOutcome::Won) => Shape::Rock,
        (Shape::Paper, GameOutcome::Won) => Shape::Scissors,
        (Shape::Rock, GameOutcome::Won) => Shape::Paper,
        (Shape::Rock, GameOutcome::Lost) => Shape::Scissors,
        (Shape::Scissors, GameOutcome::Lost) => Shape::Paper,
        (Shape::Paper, GameOutcome::Lost) => Shape::Rock
    }
}