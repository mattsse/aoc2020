use std::borrow::Cow;

use aoc2020::read_to_string;

fn main() {
    let input = read_to_string("input/day04");
    let mut lines = input.lines();
    let mut validators = Vec::new();
    let mut validator = PassportValidator::default();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            let mut v = PassportValidator::default();
            std::mem::swap(&mut validator, &mut v);
            validators.push(v);
        } else {
            validator.read_line(line);
        }
    }
    validators.push(validator);

    println!(
        "Found {} valid passports",
        validators.iter().filter(|p| p.is_valid_part2()).count()
    );
}

#[derive(Default)]
struct PassportValidator<'a> {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hcl: Option<Cow<'a, str>>,
    hgt: Option<Cow<'a, str>>,
    ecl: Option<Cow<'a, str>>,
    pid: Option<Cow<'a, str>>,
    cid: Option<Cow<'a, str>>,
}

impl<'a> PassportValidator<'a> {
    fn is_valid_part1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hcl.is_some()
            && self.hgt.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid_part2(&self) -> bool {
        self.byr
            .map(|byr| 1920 <= byr && byr <= 2002)
            .unwrap_or_default()
            && self
                .iyr
                .map(|iyr| 2010 <= iyr && iyr <= 2020)
                .unwrap_or_default()
            && self
                .eyr
                .map(|eyr| 2020 <= eyr && eyr <= 2030)
                .unwrap_or_default()
            && self
                .hgt
                .as_ref()
                .map(|hgt| {
                    if hgt.ends_with("cm") {
                        hgt[..hgt.len() - 2]
                            .parse::<u32>()
                            .map(|n| 150 <= n && n <= 193)
                            .unwrap_or_default()
                    } else if hgt.ends_with("in") {
                        hgt[..hgt.len() - 2]
                            .parse::<u32>()
                            .map(|n| 59 <= n && n <= 76)
                            .unwrap_or_default()
                    } else {
                        false
                    }
                })
                .unwrap_or_default()
            && self
                .hcl
                .as_ref()
                .map(|hcl| {
                    hcl.starts_with('#')
                        && hcl
                            .chars()
                            .skip(1)
                            .all(|c| matches!(c, '0'..='9'  | 'a'..='f'))
                        && hcl.len() == 7
                })
                .unwrap_or_default()
            && self
                .ecl
                .as_ref()
                .map(|ecl| {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_ref())
                })
                .unwrap_or_default()
            && self
                .pid
                .as_ref()
                .map(|pid| pid.chars().all(|c| c.is_digit(10)) && pid.len() == 9)
                .unwrap_or_default()
    }

    fn read_line(&mut self, info: &'a str) {
        for field in info.split_whitespace() {
            let val = &field[4..];
            match &field[..3] {
                "byr" => self.byr = val.parse().ok(),
                "iyr" => self.iyr = val.parse().ok(),
                "eyr" => self.eyr = val.parse().ok(),
                "hcl" => self.hcl = Some(val.into()),
                "hgt" => self.hgt = Some(val.into()),
                "ecl" => self.ecl = Some(val.into()),
                "pid" => self.pid = Some(val.into()),
                "cid" => self.cid = Some(val.into()),
                s => panic!("Unknown field {}", s),
            }
        }
    }
}
