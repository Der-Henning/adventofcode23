use std::collections::HashMap;
use super::utils::input;

fn games() -> Vec<(i32, Vec<Vec<(String, i32)>>)> {
    input(&2)
        .split("\n")
        .map(|game: &str| {
            let arr: Vec<&str> = game.split(":").collect();
            let game_num: i32 = arr[0].replace(char::is_alphabetic, "").trim().parse().unwrap();
            let draws: Vec<Vec<(String, i32)>> = arr[1]
                .split(";")
                .map(|draw: &str| {
                    draw.split(",")
                        .map(|col: &str| {
                            let arr: Vec<&str> = col.trim().split(" ").collect();
                            let count: i32 = arr[0].parse().unwrap();
                            let color: String = arr[1].to_string();
                            (color, count)
                        })
                        .collect::<Vec<(String, i32)>>()
                })
                .collect::<Vec<Vec<(String, i32)>>>();
            (game_num, draws)
        })
        .collect::<Vec<(i32, Vec<Vec<(String, i32)>>)>>()
}

pub fn task1() -> i32 {
    let color_size: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("blue", 14),
        ("green", 13),
    ]);
    games().iter()
        .filter(|(_, game)| 
            game.iter().flatten()
            .all(|(color, count)| count <= &color_size[color.as_str()]))
        .map(|(game_num, _)| game_num)
        .sum::<i32>()
}

pub fn task2() -> i32 {
    games().iter().map(|(_, game)| {
        let mut num_colors: HashMap<String, i32> = HashMap::new();
        for (color, count) in game.iter().flatten() {
            if !num_colors.contains_key(color) || count > &num_colors[color] {
                num_colors.insert(color.clone(), *count);
            }
        }
        num_colors.values().product::<i32>()
    }).sum::<i32>()
}
