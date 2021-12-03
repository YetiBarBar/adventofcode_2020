use adventofcode_2020::utils::read_lines;

pub fn main() {
    let values: Vec<usize> = read_lines("day_2020_1.data")
        .unwrap()
        .map(Result::unwrap)
        .map(|l| l.trim().parse::<usize>())
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {:?}", process(&values, 2, 2020));
    println!("Part 2: {:?}", process(&values, 3, 2020));
}

#[must_use]
pub fn process(values: &[usize], k: usize, goal: usize) -> Option<usize> {
    use itertools::Itertools;
    values
        .iter()
        .combinations(k)
        .find(|v| v.iter().copied().sum::<usize>() == goal)
        .map(|v| v.iter().copied().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part_1() {
        let values: &[usize] = &[1721, 979, 366, 299, 675, 1456];
        let step1_tst = process(values, 2, 2020);
        assert_eq!(step1_tst, Some(514579usize));
    }

    #[test]
    fn test_day1_part_2() {
        let values: &[usize] = &[1721, 979, 366, 299, 675, 1456];
        let step2_tst = process(values, 3, 2020);
        assert_eq!(step2_tst, Some(241861950usize));
    }
}
