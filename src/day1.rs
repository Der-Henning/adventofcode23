use crate::utils::input;

fn rows() -> Vec<String> {
    input(&1).split("\n").map(|line: &str| line.to_string()).collect()
}

fn get_number(s: &String) -> i32 {
    let arr: Vec<char> = s.replace(char::is_alphabetic, "").chars().collect();
    format!("{}{}", arr[0], arr[arr.len()-1]).parse::<i32>().unwrap()
}

pub fn task1() -> i32 {
    rows().iter().map(|row: &String| get_number(row)).sum()
}

pub fn task2() -> i32 {
    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    rows().iter().map(|row: &String| {
        let mut x: String = row.to_string();
        for (i, number) in numbers.iter().enumerate() {
            x = x.replace(number, format!("{}{}{}", number, i, number).as_str());
        }
        get_number(&x)
    }).sum()
}
