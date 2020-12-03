use aoc2020::read_to_string;

fn main() {
    let input = read_to_string("input/day02");
    let num_valid = input.lines().filter(|line| {
        let mut split = line.split(|c: char| c.is_whitespace() || c == '-' || c == ':').filter(|s| !s.is_empty());
        let lo = split.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
        let hi = split.next().and_then(|s| s.parse::<usize>().ok()).unwrap();

        // is_valid_password_part1(lo, hi, split.next().and_then(|s| s.chars().next()).unwrap(), split.next().unwrap())
        is_valid_password_part2(lo, hi, split.next().unwrap(), split.next().unwrap())
    }).count();
    println!("valid passwords {}", num_valid);
}


fn is_valid_password_part1(lo: usize, hi: usize, c: char, pass: &str) -> bool {
    let num = pass.chars().filter(|p| *p == c).count();
    lo <= num && num <= hi
}

fn is_valid_password_part2(lo: usize, hi: usize, c: &str, pass: &str) -> bool {
    (&pass[lo-1..lo] == c) ^ (&pass[hi-1..hi] == c)
}
