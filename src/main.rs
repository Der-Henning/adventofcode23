mod utils;
mod day1;
mod day2;
mod day3;

fn print_results(day: i32, task1: i32, task2: i32) {
    println!("Day {}:\nResult 1: {}\nResult 2: {}", day, task1, task2);
    println!("------------------");
}

fn main() {
    print_results(1, day1::task1(), day1::task2());
    print_results(2, day2::task1(), day2::task2());
    print_results(3, day3::task1(), day3::task2());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        assert_eq!(day1::task1(), 55386);
        assert_eq!(day1::task2(), 54824);
    }

    #[test]
    fn day2() {
        assert_eq!(day2::task1(), 1931);
        assert_eq!(day2::task2(), 83105);
    }

    #[test]
    fn day3() {
        assert_eq!(day3::task1(), 498559);
        assert_eq!(day3::task2(), 72246648);
    }
}
