use std::fs::File;
use std::error::Error;
use std::env;
use std::io::{BufRead, BufReader};

fn main()
{
    let args: Vec<String> = env::args().collect();

    // parse args
    let inputName = &args[1];
    let mut learnRate: f32 = args[2].parse().unwrap();
    let mut discount: f32 = args[3].parse().unwrap();
    let iterations: i16 = args[4].parse().unwrap();

    // open file
    let mut reader = BufReader::new(File::open(inputName).unwrap());

    // get number of lines and columns
    let mut numLine = String::new();
    reader.read_line(&mut numLine);
    let sizes: Vec<usize> = numLine.split(char::is_whitespace)
        .take(2)
        .map(|n| n.parse().unwrap())
        .collect();
    let mut map: Vec<Vec<char>> = reader.lines()
        .take(sizes[0])
        .map(|l| l.unwrap().chars()
             .take(sizes[1])
             .collect())
        .collect();

}
