use std::fs;

pub fn input(day: &i32) -> String {
    fs::read_to_string(format!("src/data/input_{}.txt", day)).unwrap()
}