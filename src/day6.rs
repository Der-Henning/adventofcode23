use super::utils::input;
use std::iter::zip;


fn get_win_count(time: f64, distance: f64) -> usize {
    let a: f64 = time / 2.0;
    let b: f64 = (time.powi(2) / 4.0 - distance).sqrt();
    (a + b).floor() as usize - (a - b).floor() as usize
}


pub fn task1() -> usize {
    let input: Vec<Vec<usize>> = input(&6).split("\n")
        .map(|x: &str| x.split(":")
            .nth(1).unwrap()
            .split_whitespace()
            .map(|x: &str| x.parse::<usize>().unwrap()).collect()
        ).collect::<Vec<Vec<usize>>>();
    zip(input[0].clone(), input[1].clone())
        .map(|race: (usize, usize)| 
            get_win_count(race.0 as f64, race.1 as f64)
        ).product::<usize>()
}


pub fn task2() -> usize {
    let input: Vec<usize> = input(&6).split("\n")
        .map(|x: &str| x
            .split(":")
            .nth(1).unwrap()
            .replace(" ", "")
            .parse().unwrap()
        ).collect::<Vec<usize>>();
    get_win_count(input[0] as f64, input[1] as f64)
}
