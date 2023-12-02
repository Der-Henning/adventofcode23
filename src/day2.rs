use crate::utils::input;

pub fn task1() {
    let input: String = input(&2);
    let games: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let red: i32 = 12;
    let blue: i32 = 14;
    let green: i32 = 13;
    let mut result: i32 = 0;
    for (i, game) in games.iter().enumerate() {
        let draws: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
        let mut possible: bool = true;
        for draw in draws.iter() {
            for col in draw.split(",").collect::<Vec<&str>>().iter() {
                let x = col.replace(char::is_alphabetic, "").trim().to_string().parse::<i32>().unwrap();
                if col.contains("red") && x > red {
                    possible = false;
                }
                if col.contains("blue") && x > blue {
                    possible = false;
                }
                if col.contains("green") && x > green {
                    possible = false;
                }
            }
        }
        if possible {
            result += i as i32 + 1;
        }
    }
    println!("Result 1: {:?}", result);
}

pub fn task2() {
    let input: String = input(&2);
    let games: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let mut result: i32 = 0;
    for game in games.iter() {
        let mut reds = 0;
        let mut blues = 0;
        let mut greens = 0;
        let draws: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>();
        for draw in draws.iter() {
            for col in draw.split(",").collect::<Vec<&str>>().iter() {
                let x = col.replace(char::is_alphabetic, "").trim().to_string().parse::<i32>().unwrap();
                if col.contains("red") && x > reds {
                    reds = x;
                }
                if col.contains("blue") && x > blues {
                    blues = x;
                }
                if col.contains("green") && x > greens {
                    greens = x;
                }
            }
        }
        result += reds * blues * greens;
    }
    println!("Result 2: {:?}", result);
}