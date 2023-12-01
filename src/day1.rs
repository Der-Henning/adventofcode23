use crate::utils::input;

pub fn task1() {
    let mut result: i32 = 0;
    for row in input(&1).split("\n").collect::<Vec<&str>>().iter() {
        let a: Vec<char> = row.replace(char::is_alphabetic, "").chars().collect::<Vec<char>>();
        result += format!("{}{}", a[0], a[a.len()-1]).parse::<i32>().unwrap();
        // println!("{:?}", format!("{}{}", a[0], a[a.len()-1]).parse::<i32>().unwrap());
    }
    println!("Result 1: {:?}", result);
}

pub fn task2() {
    let mut result: i32 = 0;
    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for row in input(&1).split("\n").collect::<Vec<&str>>().iter() {
        let mut x: String = row.to_string();
        for (i, number) in numbers.iter().enumerate() {
            x = x.replace(number, format!("{}{}{}", number, i, number).as_str());
        }
        let a: Vec<char> = x.replace(char::is_alphabetic, "").chars().collect::<Vec<char>>();
        result += format!("{}{}", a[0], a[a.len()-1]).parse::<i32>().unwrap();
        // println!("{:?}", format!("{}{}", a[0], a[a.len()-1]).parse::<i32>().unwrap());
    }
    println!("Result 2: {:?}", result);
}