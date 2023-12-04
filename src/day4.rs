use super::utils::input;
use std::collections::{HashSet, HashMap};


struct Card {
    card_num: i32,
    win_nums: Vec<i32>,
    have_nums: Vec<i32>
}


fn split_nums(txt: &str) -> Vec<i32> {
    txt.trim()
    .split_whitespace()
    .map(|x: &str| x.parse::<i32>().unwrap())
    .collect::<Vec<i32>>()
}


fn parse_input() -> Vec<Card> {
    input(&4).split("\n").map(|line: &str| {
        let a: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let card_num: i32 = a[0].replace(char::is_alphabetic, "").trim().parse().unwrap();
        let b: Vec<&str> = a[1].split("|").collect::<Vec<&str>>();
        let win_nums: Vec<i32> = split_nums(&b[0]);
        let have_nums: Vec<i32> = split_nums(&b[1]);
        Card {card_num, win_nums, have_nums}
    })
    .collect::<Vec<Card>>()
}


fn get_matches(card: &Card) -> i32 {
    HashSet::<i32>::from_iter(card.win_nums.clone())
        .intersection(&HashSet::<i32>::from_iter(card.have_nums.clone()))
        .count() as i32
}


pub fn task1() -> i32 {
    parse_input().iter().map(|card: &Card| {
        let matches: u32 = get_matches(card) as u32;
        if matches == 0 {0} else {2_i32.pow(matches-1)}
    }).sum::<i32>()
}


fn get_recursive_matches(cards: &Vec<Card>, card_num: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(matches) = cache.get(&card_num) {
        return *matches;
    }
    let card: &Card = cards.iter().find(|card: &&Card| card_num == card.card_num).unwrap();
    let mut matches: i32 = get_matches(card);
    if matches > 0 {
        matches += (card_num + 1..card_num + matches + 1)
            .map(|i: i32| get_recursive_matches(cards, i, cache))
            .sum::<i32>();
    }
    cache.insert(card_num, matches);
    matches
}


pub fn task2() -> i32 {
    let cards: Vec<Card> = parse_input();
    let mut cache: HashMap<i32, i32> = HashMap::new();
    cards.iter()
        .map(|card: &Card| get_recursive_matches(&cards, card.card_num, &mut cache))
        .sum::<i32>() + cards.len() as i32
}
