use std::collections::HashSet;

use itertools::Itertools;
use num::Float;

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.into_iter().collect::<HashSet<i32>>().into_iter()
    .sorted().map(|n| n.to_string()).collect::<String>().parse().expect("Expected all digits to be num.")
}

fn power_of_two(x: u64) -> bool {
    x.is_power_of_two()
}

fn power_of_two_1(x: u64) -> bool {
    match x {
        0 => false,
        1 => true,
        _ => {
            let log_of_x = (x as f64).log2();
            log_of_x.floor() == log_of_x.ceil()
        }
    }

}

fn main(){}