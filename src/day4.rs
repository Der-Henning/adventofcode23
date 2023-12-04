use super::utils::input;
use std::collections::{HashSet, HashMap};


fn parse_input() -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    input(&4).split("\n").map(|line: &str| {
        let a: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let card_num: i32 = a[0].replace(char::is_alphabetic, "").trim().parse().unwrap();
        let b: Vec<&str> = a[1].split("|").collect::<Vec<&str>>();
        let win_nums: Vec<i32> = b[0].trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let have_nums: Vec<i32> = b[1].trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        (card_num, win_nums, have_nums)
    })
    .collect::<Vec<(i32, Vec<i32>, Vec<i32>)>>()
}

fn get_matches(win_nums: &Vec<i32>, have_nums: &Vec<i32>) -> i32 {
    HashSet::<i32>::from_iter(win_nums.clone())
        .intersection(&HashSet::<i32>::from_iter(have_nums.clone()))
        .map(|i: &i32| *i)
        .collect::<Vec<i32>>()
        .len() as i32
}


pub fn task1() -> i32 {
    parse_input().iter().map(|(_, win_nums, have_nums)| {
        let matches: u32 = get_matches(win_nums, have_nums) as u32;
        if matches == 0 {0} else {2_i32.pow(matches-1)}
    }).sum::<i32>()
}


fn get_recursive_matches(cards: &Vec<(i32, Vec<i32>, Vec<i32>)>, card_num: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if let Some(matches) = memo.get(&card_num) {
        return *matches;
    }
    let card: &(i32, Vec<i32>, Vec<i32>) = cards.iter().find(|(num, _, _)| num == &card_num).unwrap();
    let mut matches: i32 = get_matches(&card.1, &card.2);
    if matches > 0 {
        matches += (card_num + 1..card_num + matches + 1)
            .map(|i: i32| get_recursive_matches(cards, i, memo))
            .sum::<i32>();
    }
    memo.insert(card_num, matches);
    matches
}


pub fn task2() ->i32 {
    let cards: Vec<(i32, Vec<i32>, Vec<i32>)> = parse_input();
    
    let mut memo: HashMap<i32, i32> = HashMap::new();
    cards.iter()
        .map(|(card_num, _, _)| get_recursive_matches(&cards, *card_num, &mut memo))
        .sum::<i32>() + cards.len() as i32
}
