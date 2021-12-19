use adventofcode_tooling::read_lines_to_vec_t;

pub fn main() {
    let values: Vec<isize> = read_lines_to_vec_t("day_2020_25.data");

    println!("Part 1: {:?}", part_1(&values));
}

static MODULUS: isize = 20_201_227;

#[must_use]
pub fn find_loop_size(value: isize) -> isize {
    let mut loop_size = 0;
    let mut result = 1;
    while result != value {
        loop_size += 1;
        result = (result * 7) % MODULUS;
    }
    loop_size
}

#[must_use]
pub fn part_1(values: &[isize]) -> isize {
    let loop_size = find_loop_size(values[1]);
    (0..loop_size).fold(1, |result, _| result * values[0] % MODULUS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part_1() {
        let values: &[isize] = &[5764801, 17807724];
        assert_eq!(part_1(values), 14897079);
    }
}
