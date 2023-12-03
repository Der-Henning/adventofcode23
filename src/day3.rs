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
                    number_arr.push(Number {
                        value: current_num_string.parse().unwrap(),
                        row_index: row_index as i32,
                        start_index: current_start_index,
                        end_index: if char_index == row.len()-1 { char_index as i32 } else { char_index as i32 - 1 }
                    });
                    current_num_string = String::new();
                }
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
    (number_arr, symbol_arr)
}


fn is_adjacent(number: &Number, symbol: &Symbol) -> bool {
    (number.row_index-1..number.row_index+2).contains(&symbol.row_index) &&
    (number.start_index-1..number.end_index+2).contains(&symbol.index)
}


pub fn task1() {
    let (numbers, symbols) = parse_input();
    let result: i32 = numbers.iter().filter(|number: &&Number| {
        symbols.iter().any(|symbol: &Symbol| is_adjacent(number, symbol))
    }).map(|number: &Number| number.value).sum::<i32>();
    println!("Result 1: {:?}", result);
}


pub fn task2() {
    let (numbers, symbols) = parse_input();
    let result: i32 = symbols.iter()
        .filter(|symbol: &&Symbol| symbol.value == '*')
        .map(|symbol: &Symbol| {
            numbers.iter()
            .filter(|number: &&Number| is_adjacent(number, symbol))
        })
        .filter(|adjacent_numbers | adjacent_numbers.clone().count() as i32 == 2)
        .map(|adjacent_numbers | adjacent_numbers.map(|number: &Number| number.value).product::<i32>())
        .sum::<i32>();
    println!("Result 2: {:?}", result)
}
