use super::utils::input;


fn get_distances_sum(boost_factor: &usize) -> usize {
    let universe: Vec<Vec<char>> = input(&11).split("\n").map(|line: &str| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let empty_rows: Vec<usize> = universe.iter()
        .enumerate()
        .filter(|line: &(usize, &Vec<char>)| line.1.iter().all(|c: &char| c == &'.'))
        .map(|(x, _)| x).collect();
    let emtpy_cols: Vec<usize> = universe[0].iter()
        .enumerate()
        .filter(|(y, _)| universe.iter().all(|line: &Vec<char>| line[*y] == '.'))
        .map(|(y, _)| y).collect();

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    universe.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, c)| {
            if c == &'#' {
                galaxies.push((x, y));
            }
        });
    });

    galaxies.iter().enumerate().flat_map(|(idx, galaxy)| {
        galaxies[idx + 1..].iter().map(|other_galaxy: &(usize, usize)| {
            let range_x: std::ops::Range<usize> = if galaxy.0 <= other_galaxy.0 {galaxy.0..other_galaxy.0} else {other_galaxy.0..galaxy.0};
            let range_y: std::ops::Range<usize> = if galaxy.1 <= other_galaxy.1 {galaxy.1..other_galaxy.1} else {other_galaxy.1..galaxy.1};
            let add_x: usize = empty_rows.iter().filter(|x: &&usize| range_x.contains(x)).count() * (boost_factor - 1);
            let add_y: usize = emtpy_cols.iter().filter(|y: &&usize| range_y.contains(y)).count() * (boost_factor - 1);
            ((galaxy.0 as isize - other_galaxy.0 as isize).abs() + (galaxy.1 as isize - other_galaxy.1 as isize).abs()) as usize + add_x + add_y
        }).collect::<Vec<usize>>()
    }).sum()
}


pub fn task1() -> usize {
    get_distances_sum(&2)
}


pub fn task2() -> usize {
    get_distances_sum(&1000000)
}
