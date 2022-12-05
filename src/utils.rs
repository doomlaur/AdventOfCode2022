use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};


// Opens a file for the requested day and depending on whether it's the test input or not.
// Note: the file might not exist! (but should, unless I did something stupid)
pub fn open_file(day: u8, load_test_file: bool) -> PathBuf {
    let day_name = format!("Day{}", day);
    let filename = if load_test_file { format!("{} input test.txt", day_name) } else { format!("{} input.txt", day_name) };
    let full_path = format!("Data/{}", filename);
    Path::new(full_path.as_str()).to_path_buf()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(filename: &Path) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}