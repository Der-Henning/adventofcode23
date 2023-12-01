use std::fs;

pub fn input(day: &i32) -> String {
    let input: String = fs::read_to_string(format!("src/data/input_{}.txt", day)).unwrap();
    return input;
}