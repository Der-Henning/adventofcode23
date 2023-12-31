mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

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
    print_results(7, day7::task1(), day7::task2());
    print_results(8, day8::task1(), day8::task2());
    print_results(9, day9::task1(), day9::task2());
    print_results(10, day10::task1(), day10::task2());
    print_results(11, day11::task1(), day11::task2());
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

    #[test]
    fn day7() {
        assert_eq!(day7::task1(), 246163188);
        assert_eq!(day7::task2(), 245794069);
    }

    #[test]
    fn day8() {
        assert_eq!(day8::task1(), 14893);
        assert_eq!(day8::task2(), 10241191004509);
    }

    #[test]
    fn day9() {
        assert_eq!(day9::task1(), 2105961943);
        assert_eq!(day9::task2(), 1019);
    }

    #[test]
    fn day10() {
        assert_eq!(day10::task1(), 6806);
        assert_eq!(day10::task2(), 449);
    }

    #[test]
    fn day11() {
        assert_eq!(day11::task1(), 9509330);
        assert_eq!(day11::task2(), 635832237682);
    }
}
