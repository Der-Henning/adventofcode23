use crate::utils::input;

#[derive(Clone)]
struct Number {
    value: i32,
    row_index: i32,
    start_index: i32,
    end_index: i32
}

struct Symbol {
    value: char,
    row_index: i32,
    index: i32
}


fn parse_input() -> (Vec<Number>, Vec<Symbol>) {
    let mut number_arr: Vec<Number> = Vec::new();
    let mut symbol_arr: Vec<Symbol> = Vec::new();
    for (row_index, row) in input(&3).split("\n").enumerate() {
        let mut current_num_string: String = String::new();
        let mut current_start_index: i32 = 0;
        for (char_index, c) in row.chars().enumerate() {
            if c.is_numeric() {
                if current_num_string.len() == 0 {
                    current_start_index = char_index as i32;
                }
                current_num_string.push(c);
            }
            if !c.is_numeric() || char_index == row.len()-1 {
                if current_num_string.len() > 0 {
                    let current_num: i32 = current_num_string.parse().unwrap();
                    let end_index: i32 = if char_index == row.len()-1 { char_index as i32 } else { char_index as i32 - 1 };
                    number_arr.push(Number {
                        value: current_num,
                        row_index: row_index as i32,
                        start_index: current_start_index,
                        end_index: end_index
                    });
                    current_num_string = String::new();
                }
                if !c.is_numeric() && c != '.' {
                    symbol_arr.push(Symbol {
                        value: c,
                        row_index: row_index as i32,
                        index: char_index as i32
                    });
                }
            }
        }
    }
    (number_arr, symbol_arr)
}


pub fn task1() {
    let (numbers, symbols) = parse_input();
    let mut final_numbers: Vec<Number> = Vec::new();
    'outer: for number in numbers {
        for symbol in &symbols {
            if (number.row_index-1..number.row_index+2).contains(&symbol.row_index) &&
               (number.start_index-1..number.end_index+2).contains(&symbol.index) {
                final_numbers.push(number.clone());
                continue 'outer;
            }
        }
    }
    let result: i32 = final_numbers.iter().map(|number: &Number| number.value).sum();
    println!("Result 1: {:?}", result);
}


pub fn task2() {
    let (numbers, symbols) = parse_input();
    let mut result = 0;
    for symbol in symbols {
        if symbol.value == '*' {
            let mut adjecent_numbers: Vec<Number> = Vec::new();
            for number in &numbers {
                if (number.row_index-1..number.row_index+2).contains(&symbol.row_index) &&
                   (number.start_index-1..number.end_index+2).contains(&symbol.index) {
                    adjecent_numbers.push(number.clone());
                }
            }
            if adjecent_numbers.len() == 2 {
                result += adjecent_numbers.iter().map(|number: &Number| number.value).product::<i32>();
            }
        }
    }
    println!("Result 2: {:?}", result)
}
