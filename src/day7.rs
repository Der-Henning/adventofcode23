use super::utils::input;
use itertools::Itertools;
use std::cmp::Ordering;


struct Hand {
    positions: Vec<usize>,
    groups: Vec<Vec<usize>>,
    bid: usize
}


fn get_positions(order: &String, hand: &String) -> Vec<usize> {
    hand.chars().map(|x: char| 
        order.chars().position(|y: char| y == x).unwrap()
    ).collect::<Vec<usize>>()
}


fn get_groups(x: Vec<usize>) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = x.into_iter()
        .sorted_unstable()
        .group_by(|e: &usize| *e)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    res.sort_unstable_by(|a: &Vec<usize>, b: &Vec<usize>| b.len().cmp(&a.len()));
    res
}


fn compare(a: &Hand, b: &Hand) -> Ordering {
    let mut result: Ordering = b.groups.len().cmp(&a.groups.len());
    if result != Ordering::Equal {
        return result;
    }
    result = a.groups[0].len().cmp(&b.groups[0].len());
    if result != Ordering::Equal {
        return result;
    }
    let mut i: usize = 0;
    while result == Ordering::Equal && i < a.positions.len() {
        result = a.positions[i].cmp(&b.positions[i]);
        i += 1;
    }
    return result;
}


fn get_inputs() -> Vec<(String, usize)> {
    input(&7).split("\n").map(|line: &str| {
        let a: Vec<&str> = line.split_whitespace().collect();
        (a[0].to_string(), a[1].to_string().parse::<usize>().unwrap())}
    ).collect::<Vec<(String, usize)>>()
}


pub fn task1() -> usize {
    let mut hands: Vec<Hand> = get_inputs().iter().map(|x| {
        let order: String = String::from("23456789TJQKA");
        let positions: Vec<usize> = get_positions(&order, &x.0);
        let groups: Vec<Vec<usize>> = get_groups(positions.clone());
        Hand {positions, groups, bid: x.1}
    }).collect();
    hands.sort_unstable_by(|a: &Hand, b: &Hand| compare(&a, &b));
    hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum::<usize>()
}


fn get_stronges_joker(order: &String, hand: &String) -> String {
    if !hand.contains("J") {
        return hand.clone();
    }
    let mut hands: Vec<String> = order.chars().map(|x: char| 
        hand.replace("J", &x.to_string())).collect();
        hands.sort_unstable_by(|a: &String, b: &String| {
        let a_pos: Vec<usize> = get_positions(&order, &a);
        let a_groups: Vec<Vec<usize>> = get_groups(a_pos.clone());
        let b_pos: Vec<usize> = get_positions(&order, &b);
        let b_groups: Vec<Vec<usize>> = get_groups(b_pos.clone());
        let mut result: Ordering = b_groups.len().cmp(&a_groups.len());
        if result != Ordering::Equal {
            return result;
        }
        result = a_groups[0].len().cmp(&b_groups[0].len());
        return result;
    });
    hands[hands.len() - 1].clone()
}


pub fn task2() -> usize {
    let mut hands: Vec<Hand> = get_inputs().iter().map(|x: &(String, usize)| {
        let order: String = String::from("J23456789TQKA");
        let stronges_joker: String = get_stronges_joker(&order, &x.0);
        let positions: Vec<usize> = get_positions(&order, &x.0);
        let positions_joker: Vec<usize> = get_positions(&order, &stronges_joker);
        let groups: Vec<Vec<usize>> = get_groups(positions_joker.clone());
        Hand {positions, groups, bid: x.1}
    }).collect::<Vec<Hand>>();
    hands.sort_unstable_by(|a: &Hand, b: &Hand| compare(&a, &b));
    hands.iter().enumerate().map(|(i, x)| (i + 1) * x.bid).sum::<usize>()
}
