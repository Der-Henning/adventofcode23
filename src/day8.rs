use super::utils::input;
use std::collections::HashMap;


fn read_input() -> (String, HashMap<String, (String, String)>) {
    let data: Vec<String> = input(&8).split("\n\n").map(|x: &str| x.to_string()).collect();
    let instructions: String = data[0].to_string();
    (instructions, HashMap::from_iter(data[1].split("\n").map(|line: &str| {
        let a: Vec<&str> = line.split("=").collect();
        let key: String = a[0].trim().to_string();
        let b: Vec<String> = a[1].split(",").map(|x: &str| x.chars().filter(|x| x.is_alphanumeric()).collect()).collect();
        (key, (b[0].clone(), b[1].clone()))
    })))
}


pub fn task1() -> usize {
    let (instructions, map): (String, HashMap<String, (String, String)>) = read_input();
    let mut current_node: String = String::from("AAA");
    let mut current_inst: usize = 0;
    let mut steps: usize = 0;
    while current_node != "ZZZ" {
        let (a, b): (String, String) = map.get(&current_node).unwrap().clone();
        if current_inst == instructions.len() {
            current_inst = 0;
        }
        current_node = if {instructions.chars().nth(current_inst).unwrap()} == 'L' {a} else {b};
        current_inst += 1;
        steps += 1;
    }
    steps
}


fn prime_factorization(n: usize) -> Vec<usize> {
    let mut n: usize = n;
    let mut factors: Vec<usize> = Vec::new();
    let mut i: usize = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}


fn get_smallest_common_multiple(vec: Vec<usize>) -> usize {
    let mut factors: HashMap<usize, usize> = HashMap::new();
    vec.iter().for_each(|x: &usize| {
        let mut prime_factors: HashMap<usize, usize> = HashMap::new();
        prime_factorization(*x).iter().for_each(|y: &usize| {
            if prime_factors.contains_key(y) {
                prime_factors.insert(*y, prime_factors.get(y).unwrap() + 1);
            } else {
                prime_factors.insert(*y, 1);
            }
        });
        prime_factors.iter().for_each(|(key, value)| {
            if factors.contains_key(key) {
                if factors.get(key).unwrap() < value {
                    factors.insert(*key, *value);
                }
            } else {
                factors.insert(*key, *value);
            }
        });
    });
    let mut result: usize = 1;
    factors.iter().for_each(|(key, value)| {
        result *= key.pow(*value as u32);
    });
    result
}


pub fn task2() -> usize {
    let (instructions, map): (String, HashMap<String, (String, String)>) = read_input();
    let start_nodes: Vec<String> = map.keys().filter(|x: &&String| x.chars().nth(2).unwrap() == 'A').map(|x: &String| x.clone()).collect();

    let mut distances: HashMap<(String, usize), (String, usize)> = HashMap::new();

    let mut start_steps: HashMap<String, Vec<usize>> = HashMap::new();
    
    start_nodes.iter().for_each(|start_node: &String| {
        let mut current_node: String = start_node.clone();
        let mut current_start_node: String = start_node.clone();
        let mut current_inst: usize = 0;
        let mut steps: usize = 0;
        let mut steps_vec: Vec<usize> = Vec::new();
        loop {
            while current_node.chars().nth(2).unwrap() != 'Z' {
                let (a, b): (String, String) = map.get(&current_node).unwrap().clone();
                if current_inst == instructions.len() {
                    current_inst = 0;
                }
                current_node = if {instructions.chars().nth(current_inst).unwrap()} == 'L' {a} else {b};
                current_inst += 1;
                steps += 1;
            }
            if distances.contains_key(&(current_start_node.clone(), current_inst - 1)) {
                break;
            }
            distances.insert((current_start_node.clone(), current_inst - 1), (current_node.clone(), steps));
            current_start_node = current_node.clone();
            steps_vec.push(steps);
        }
        start_steps.insert(start_node.clone(), steps_vec);
    });
    // println!("{:?}", distances);

    // println!("{:?}", start_steps);

    let base: Vec<usize> = start_steps.values().map(|x: &Vec<usize>| x[0]).collect();

    get_smallest_common_multiple(base)

}
