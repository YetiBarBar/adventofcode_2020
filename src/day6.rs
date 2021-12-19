use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

#[must_use]
pub fn part_1(data: &[String]) -> usize {
    data.iter()
        .map(|s| {
            s.trim()
                .chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[must_use]
pub fn part_2(data: &[String]) -> usize {
    data.iter()
        .map(|group| {
            let mut hmap = HashMap::<char, usize>::new();
            for line in group.lines() {
                let chars = line.chars().collect::<HashSet<char>>();
                for c in &chars {
                    *hmap.entry(*c).or_insert(0) += 1;
                }
            }
            hmap.iter()
                .filter(|(_, count)| **count == group.lines().count())
                .count()
        })
        .sum()
}

pub fn main() {
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2020_6.data");
    let input_data = std::fs::read_to_string(filepath).unwrap();

    let groups: Vec<String> = input_data.split("\n\n").map(str::to_string).collect();

    println!("Part 1: {:?}", part_1(&groups));
    println!("Part 2: {:?}", part_2(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groups_part_1() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let groups: Vec<String> = input.split("\n\n").map(|s| str::to_string(s)).collect();
        assert_eq!(part_1(&groups), 11);
    }

    #[test]
    fn test_groups_part_2() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let groups: Vec<String> = input.split("\n\n").map(|s| str::to_string(s)).collect();
        assert_eq!(part_2(&groups), 6);
    }
}
