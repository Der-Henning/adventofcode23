use super::utils::input;


fn read_input() -> Vec<Vec<isize>> {
    input(&9).split("\n")
        .map(|line: &str| line.split_whitespace()
            .map(|x: &str| x.parse::<isize>().unwrap()
        ).collect()
    ).collect()
}


fn get_next_line(line: Vec<isize>) -> Vec<isize> {
    let mut next_line: Vec<isize> = Vec::new();
    for i in 1..line.len() {
        next_line.push(line[i]-line[i - 1]);
    }
    next_line
}


pub fn task1() -> isize {
    read_input().iter().map(|line: &Vec<isize>| {
        let mut last_column: Vec<isize> = Vec::from([line[line.len() - 1]]);
        let mut next_line: Vec<isize> = get_next_line(line.clone());
        while next_line.iter().any(|x: &isize| x != &0) {
            last_column.push(next_line[next_line.len() - 1]);
            next_line = get_next_line(next_line);
        }
        last_column.iter().rev().fold(0, |acc: isize, x: &isize| acc + x)
    }).sum::<isize>()
}

pub fn task2() -> isize {
    read_input().iter().map(|line: &Vec<isize>| {
        let mut first_column: Vec<isize> = Vec::from([line[0]]);
        let mut next_line: Vec<isize> = get_next_line(line.clone());
        while next_line.iter().any(|x: &isize| x != &0) {
            first_column.push(next_line[0]);
            next_line = get_next_line(next_line);
        }
        first_column.iter().rev().fold(0, |acc: isize, x: &isize| x - acc)
    }).sum::<isize>()
}
