use super::utils::input;
use regex::Regex;


fn read_input() -> Vec<Vec<char>> {
    input(&10).split("\n").map(|line: &str| line.chars().collect()).collect()
}


fn calc_pipe(map: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let mut current_node: (usize, usize, char) = (0, 0, 'S');
    map.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, c)| {
            if c == &'S' {current_node = (x, y, 'S');}
        })
    });
    let mut nodes: Vec<(usize, usize, char)> = Vec::from([current_node]);
    if String::from("-7J").chars().collect::<Vec<char>>().contains(&map[current_node.0][current_node.1 + 1]) {
        current_node.1 += 1;
    } else if String::from("|LJ").chars().collect::<Vec<char>>().contains(&map[current_node.0 + 1][current_node.1]) {
        current_node.0 += 1;
    } else if String::from("-LF").chars().collect::<Vec<char>>().contains(&map[current_node.0][current_node.1 - 1]) {
        current_node.1 -= 1;
    } else if String::from("|7F").chars().collect::<Vec<char>>().contains(&map[current_node.0 - 1][current_node.1]) {
        current_node.0 -= 1;
    }
    current_node.2 = map[current_node.0][current_node.1];
    nodes.push(current_node);
    while map[current_node.0][current_node.1] != 'S' {
        if map[current_node.0][current_node.1] == '|' {
            if nodes[nodes.len() - 2].0 == current_node.0 - 1 {
                current_node.0 += 1;
            } else {
                current_node.0 -= 1;
            }
        } else if map[current_node.0][current_node.1] == '-' {
            if nodes[nodes.len() - 2].1 == current_node.1 - 1 {
                current_node.1 += 1;
            } else {
                current_node.1 -= 1;
            }
        } else if map[current_node.0][current_node.1] == '7' {
            if nodes[nodes.len() - 2].1 == current_node.1 - 1 {
                current_node.0 += 1;
            } else {
                current_node.1 -= 1;
            }
        } else if map[current_node.0][current_node.1] == 'J' {
            if nodes[nodes.len() - 2].0 == current_node.0 - 1 {
                current_node.1 -= 1;
            } else {
                current_node.0 -= 1;
            }
        } else if map[current_node.0][current_node.1] == 'L' {
            if nodes[nodes.len() - 2].1 == current_node.1 + 1 {
                current_node.0 -= 1;
            } else {
                current_node.1 += 1;
            }
        } else if map[current_node.0][current_node.1] == 'F' {
            if nodes[nodes.len() - 2].0 == current_node.0 + 1 {
                current_node.1 += 1;
            } else {
                current_node.0 += 1;
            }
        }
        current_node.2 = map[current_node.0][current_node.1];
        nodes.push(current_node);
    }
    nodes
}


pub fn task1() -> usize {
    let input: Vec<Vec<char>> = read_input();
    calc_pipe(&input).len() / 2
}


pub fn task2() -> usize {
    let input: Vec<Vec<char>> = read_input();
    let pipe: Vec<(usize, usize, char)> = calc_pipe(&input);
    let map: Vec<(usize, usize, char)> = input.iter()
        .enumerate().flat_map(|(x, line)| line.iter()
            .enumerate().map(|(y, c)| (x, y, *c)
        ).collect::<Vec<(usize, usize, char)>>()
    ).collect::<Vec<(usize, usize, char)>>();
    let mut clean_map: Vec<Vec<char>> = vec![vec!['.'; input[1].len()]; input.len()];
    pipe.iter().for_each(|x: &(usize, usize, char)| {clean_map[x.0][x.1] = x.2;});
    let tiles: Vec<&(usize, usize, char)> = map.iter()
        .filter(|(x, y, _)| clean_map[*x][*y] == '.')
        .collect::<Vec<&(usize, usize, char)>>();
    // clean_map.iter().for_each(|line: &Vec<char>| println!("{:?}", line.iter().collect::<String>()));
    tiles.iter().map(|(x, y, _)| {
        if clean_map[*x][0..*y].iter().any(|c: &char| c != &'.') &&
            clean_map[0..*x].iter().map(|line: &Vec<char>| line[*y]).any(|c: char| c != '.') &&
            clean_map[*x][*y..].iter().any(|c: &char| c != &'.') &&
            clean_map[*x..].iter().map(|line: &Vec<char>| line[*y]).any(|c: char| c != '.') {
                let verticals: usize = Regex::new(r"(L-*7)|(F-*J)|(\|)").unwrap()
                    .find_iter(&clean_map[*x][0..*y].iter()
                    .collect::<String>()).count();
                let horizontals: usize = Regex::new(r"(F\|*J)|(7\|*L)|(-)").unwrap()
                    .find_iter(&clean_map[0..*x].iter().map(|line: &Vec<char>| line[*y])
                    .collect::<String>()).count();
                if verticals % 2 == 1 && horizontals % 2 == 1 {1} else {0}
        } else {0}
    }).sum()
}
