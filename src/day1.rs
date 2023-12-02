use crate::utils::input;

fn inputs() -> Vec<String> {
    input(&1).split("\n").map(|line| line.to_string()).collect()
}

fn get_number(s: &String) -> i32 {
    let arr: Vec<char> = s.replace(char::is_alphabetic, "").chars().collect();
    format!("{}{}", arr[0], arr[arr.len()-1]).parse::<i32>().unwrap()
}

pub fn task1() {
    let mut result: i32 = 0;
    for row in inputs().iter() {
        result += get_number(row);
    }
    println!("Result 1: {:?}", result);
}

pub fn task2() {
    let mut result: i32 = 0;
    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for row in inputs().iter() {
        let mut x: String = row.to_string();
        for (i, number) in numbers.iter().enumerate() {
            x = x.replace(number, format!("{}{}{}", number, i, number).as_str());
        }
        result += get_number(&x);
    }
    println!("Result 2: {:?}", result);
}
