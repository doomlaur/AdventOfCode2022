mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main()
{
    let day = 4;
    let load_test_file = true;
    
    match day
    {
        1 => day1::run(load_test_file),
        2 => day2::run(load_test_file),
        3 => day3::run(load_test_file),
        4 => day4::run(load_test_file),
        _ => {}
    }
}
