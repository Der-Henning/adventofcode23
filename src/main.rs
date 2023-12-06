mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn print_results<T: std::fmt::Debug>(day: i32, task1: T, task2: T) {
    println!("Day {:?}:\nResult 1: {:?}\nResult 2: {:?}", day, task1, task2);
    println!("------------------");
}

fn main() {
    print_results(1, day1::task1(), day1::task2());
    print_results(2, day2::task1(), day2::task2());
    print_results(3, day3::task1(), day3::task2());
    print_results(4, day4::task1(), day4::task2());
    print_results(5, day5::task1(), day5::task2());
    print_results(6, day6::task1(), day6::task2());
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

    #[test]
    fn day4() {
        assert_eq!(day4::task1(), 22193);
        assert_eq!(day4::task2(), 5625994);
    }

    #[test]
    fn day5() {
        assert_eq!(day5::task1(), 31599214);
        assert_eq!(day5::task2(), 20358599);
    }

    #[test]
    fn day6() {
        assert_eq!(day6::task1(), 512295);
        assert_eq!(day6::task2(), 36530883);
    }
}
