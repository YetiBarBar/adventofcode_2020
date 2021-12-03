use adventofcode_2020::utils::read_lines;

#[must_use]
pub fn part_1(input: &[String], horiz: usize, vert: usize) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut points = Vec::new();
    while y.lt(&input.len()) {
        let c = input[y].chars().nth(x % input[0].trim().len()).unwrap();
        points.push(c);
        x += horiz;
        y += vert;
    }
    points.iter().filter(|&&value| value == '#').count()
}

#[must_use]
pub fn part_2(input: &[String]) -> usize {
    let moves = [(1_usize, 1_usize), (3, 1), (5, 1), (7, 1), (1, 2)];
    moves.iter().map(|&(x, y)| part_1(input, x, y)).product()
}

pub fn main() {
    let values: Vec<_> = read_lines("day_2020_3.data")
        .unwrap()
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {:?}", part_1(&values, 3, 1));
    println!("Part 2: {:?}", part_2(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part_1() {
        let input = [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

        let count = part_1(&input, 3, 1);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_day2_part_2() {
        let input = [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

        let count = part_2(&input);
        assert_eq!(count, 336);
    }
}
