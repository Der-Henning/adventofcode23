use super::utils::input;


pub fn task1() -> usize {
    let mut universe: Vec<Vec<char>> = input(&11).split("\n").map(|line: &str| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut add_rows: Vec<usize> = Vec::new();
    let mut add_cols: Vec<usize> = Vec::new();
    universe.iter().enumerate().for_each(|(x, line)| {
        if line.iter().all(|c: &char| c == &'.') {
            add_rows.push(x);
        }
    });
    universe[0].iter().enumerate().for_each(|(y, _)| {
        if universe.iter().all(|line: &Vec<char>| line[y] == '.') {
            add_cols.push(y);
        }
    });

    add_rows.iter().rev().for_each(|x: &usize| {
        universe.insert(*x, vec!['.'; universe[0].len()]);
    });
    add_cols.iter().rev().for_each(|y: &usize| {
        universe.iter_mut().for_each(|line: &mut Vec<char>| {
            line.insert(*y, '.');
        });
    });
    // universe.iter().for_each(|line| {
    //     println!("{:?}", line.iter().collect::<String>());
    // });

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
            ((galaxy.0 as isize - other_galaxy.0 as isize).abs() + (galaxy.1 as isize - other_galaxy.1 as isize).abs()) as usize
        }).collect::<Vec<usize>>()
    }).sum()
}


pub fn task2() -> usize {
    let boost_factor: usize = 1000000;
    let universe: Vec<Vec<char>> = input(&11).split("\n").map(|line: &str| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut add_rows: Vec<usize> = Vec::new();
    let mut add_cols: Vec<usize> = Vec::new();
    universe.iter().enumerate().for_each(|(x, line)| {
        if line.iter().all(|c: &char| c == &'.') {
            add_rows.push(x);
        }
    });
    universe[0].iter().enumerate().for_each(|(y, _)| {
        if universe.iter().all(|line: &Vec<char>| line[y] == '.') {
            add_cols.push(y);
        }
    });

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
            let r_x: std::ops::Range<usize> = if galaxy.0 <= other_galaxy.0 {galaxy.0..other_galaxy.0} else {other_galaxy.0..galaxy.0};
            let r_y: std::ops::Range<usize> = if galaxy.1 <= other_galaxy.1 {galaxy.1..other_galaxy.1} else {other_galaxy.1..galaxy.1};
            let add_x: usize = add_rows.iter().filter(|x: &&usize| r_x.contains(x)).count() * (boost_factor - 1);
            let add_y: usize = add_cols.iter().filter(|y: &&usize| r_y.contains(y)).count() * (boost_factor - 1);
            ((galaxy.0 as isize - other_galaxy.0 as isize).abs() + (galaxy.1 as isize - other_galaxy.1 as isize).abs()) as usize + add_x + add_y
        }).collect::<Vec<usize>>()
    }).sum()
}
