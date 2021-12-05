use std::collections::HashSet;

use adventofcode_2020::utils::read_lines;

#[must_use]
pub fn part_1(data: &[String]) -> usize {
    data.iter()
        .map(|seat| decode_part_1(seat))
        .max()
        .unwrap_or_default()
}

#[must_use]
pub fn decode_part_1(data: &str) -> usize {
    decode_seat(&data[0..7]) * 8 + decode_row(&data[7..])
}

#[must_use]
pub fn decode_seat(seat: &str) -> usize {
    decode(seat, &[64, 32, 16, 8, 4, 2, 1], 'B')
}

#[must_use]
pub fn decode_row(row: &str) -> usize {
    decode(row, &[4, 2, 1], 'R')
}

#[must_use]
pub fn decode(in_data: &str, pows: &[usize], symbol: char) -> usize {
    pows.iter()
        .zip(in_data.chars())
        .map(|(value, character)| match character {
            s if s == symbol => value,
            _ => &0,
        })
        .sum()
}

#[must_use]
pub fn part_2(data: &[String]) -> usize {
    let seats: HashSet<_> = data.iter().map(|seat| decode_part_1(seat)).collect();

    println!("{}", seats.len());
    let mut candidates = vec![];
    // We are not in the first row, niether the last
    // So our Id is between 9 to (127 * 8 + 1)
    for idx in 9..(127 * 8) {
        if !seats.contains(&idx) && seats.contains(&(idx - 1)) && seats.contains(&(idx + 1)) {
            candidates.push(idx);
        }
    }

    *candidates.first().unwrap_or(&0)
}

pub fn main() {
    let values: Vec<_> = read_lines("day_2020_5.data")
        .unwrap()
        .map(Result::unwrap)
        .collect();

    println!("Part 1: {:?}", part_1(&values));
    println!("Part 2: {:?}", part_2(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valeur_seat_rows() {
        let input = "FBFBBFFRLR";
        assert_eq!(decode_part_1(input), 357);
    }
}
