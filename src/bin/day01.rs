#![allow(dead_code)]
use std::str::FromStr;

use aoc2020::read_to_string;

fn main() {
    let mut numbers = read_to_string("input/day01")
        .lines()
        .map(u32::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    numbers.sort();
    let solution = sum3(&numbers);
    let (n1, n2, n3) = solution.expect("No solution");
    println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
}

fn sum2(nums: &[u32]) -> Option<(u32, u32)> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        match nums[j] + nums[i] {
            2020 => return Some((nums[i], nums[j])),
            _n if _n > 2020 => {
                j -= 1;
                i = 0;
            }
            _ => i += 1,
        }
    }
    None
}

fn sum3(nums: &[u32]) -> Option<(u32, u32, u32)> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut k = 1;

    while k < j {
        match nums[i] + nums[j] + nums[k] {
            2020 => return Some((nums[i], nums[j], nums[k])),
            _n if _n > 2020 => {
                j -= 1;
                i = 0;
                k = 1;
            }
            _ => k += 1,
        }
    }
    None
}
