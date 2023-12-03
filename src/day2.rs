use std::collections::HashMap;

use crate::utils::input;

fn games() -> Vec<(i32, Vec<(i32, String)>)> {
    input(&2)
        .split("\n")
        .map(|game: &str| {
            let arr: Vec<&str> = game.split(":").collect();
            let game_num: i32 = arr[0].replace(char::is_alphabetic, "").trim().parse().unwrap();
            let draws = arr[1]
                .split(";")
                .flat_map(|draw: &str| {
                    draw.split(",")
                        .map(|col: &str| {
                            let arr: Vec<&str> = col.trim().split(" ").collect();
                            let count: i32 = arr[0].parse().unwrap();
                            let color: String = arr[1].to_string();
                            (count, color)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (game_num, draws)
        })
        .collect::<Vec<_>>()
}

pub fn task1() -> i32 {
    let color_size: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("blue", 14),
        ("green", 13),
    ]);
    games().iter()
        .filter(|(_, game)| 
            game.iter()
            .all(|(count, color)| count <= &color_size[color.as_str()]))
        .map(|(game_num, _)| game_num)
        .sum::<i32>()
}

pub fn task2() -> i32 {
    let mut result: i32 = 0;
    for (_, game) in games().iter() {
        let mut num_colors: HashMap<&str, i32> = HashMap::from([
            ("red", 0),
            ("blue", 0),
            ("green", 0),
        ]);
        for (count, color) in game {
            if count > &num_colors[color.as_str()] {
                num_colors.insert(color, *count);
            }
        }
        result += num_colors.values().product::<i32>();
    }
    result
}
