use std::fs::File;
use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let inputName = &args[1];
    let mut learnRate: f32 = args[2].parse().unwrap();
    let mut discount: f32 = args[3].parse().unwrap();
    let iterations: i16 = args[4].parse().unwrap();
}
