use std::str::FromStr;

use adventofcode_tooling::read_lines_to_vec_t;

#[derive(Debug)]
struct PasswdRule {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

impl FromStr for PasswdRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        // Get min and max from parts[0]
        let min_max: Vec<_> = parts[0].split('-').collect();
        let (min, max) = (min_max[0].parse().unwrap(), min_max[1].parse().unwrap());

        // Get c from parts[1]
        let c = parts[1].chars().next().unwrap();

        // Get pass
        let pass = parts[2].to_string();

        Ok(PasswdRule { min, max, c, pass })
    }
}

impl PasswdRule {
    pub fn is_valid(&self, part: usize) -> bool {
        match part {
            1 => self.is_valid_part_1(),
            2 => self.is_valid_part_2(),
            _ => unreachable!(),
        }
    }
    pub fn is_valid_part_1(&self) -> bool {
        let count = self.pass.matches(self.c).count();
        count.le(&self.max) && count.ge(&self.min)
    }

    pub fn is_valid_part_2(&self) -> bool {
        if self.min.lt(&1) || self.max.gt(&self.pass.len()) {
            return false;
        }
        let sub1 = &self.pass[self.min - 1..self.min];
        let sub2 = &self.pass[self.max - 1..self.max];
        (sub1 == self.c.to_string()) ^ (sub2 == self.c.to_string())
    }
}

#[must_use]
pub fn part_1(input: &[String]) -> usize {
    validate_for_part(input, 1)
}

#[must_use]
fn validate_for_part(input: &[String], part: usize) -> usize {
    input
        .iter()
        .filter_map(|s| PasswdRule::from_str(s).ok())
        .filter(|rule| rule.is_valid(part))
        .count()
}

#[must_use]
pub fn part_2(input: &[String]) -> usize {
    validate_for_part(input, 2)
}

pub fn main() {
    let values: Vec<_> = read_lines_to_vec_t("day_2020_2.data");

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part_1() {
        let inputs = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert!(PasswdRule::from_str(inputs[0]).unwrap().is_valid_part_1());
        assert!(!PasswdRule::from_str(inputs[1]).unwrap().is_valid_part_1());
        assert!(PasswdRule::from_str(inputs[2]).unwrap().is_valid_part_1());
        let count = inputs
            .iter()
            .map(|s| PasswdRule::from_str(s))
            .map(Result::unwrap)
            .filter(|rule| rule.is_valid_part_1())
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_day2_part_2() {
        let inputs = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        assert!(PasswdRule::from_str(inputs[0]).unwrap().is_valid_part_2());
        assert!(!PasswdRule::from_str(inputs[1]).unwrap().is_valid_part_2());
        assert!(!PasswdRule::from_str(inputs[2]).unwrap().is_valid_part_2());
        let count = inputs
            .iter()
            .map(|s| PasswdRule::from_str(s))
            .map(Result::unwrap)
            .filter(|rule| rule.is_valid_part_2())
            .count();
        assert_eq!(count, 1);
    }
}
