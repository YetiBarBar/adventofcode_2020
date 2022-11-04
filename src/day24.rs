use std::{collections::HashMap, ptr::NonNull, str::FromStr};

use adventofcode_tooling::{read_lines_to_vec_t, AocError};

#[derive(Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

enum Direction {
    South,
    North,
}

impl FromStr for Position {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ERROR: South-West is not (-1,-1) ... 6 neighbours!!!
        let (x, y, remain, res) =
            s.trim()
                .chars()
                .fold((0, 0, None, None), |(x, y, remain, err), ch| match ch {
                    's' => (x, y, Some('s'), err),
                    'n' => (x, y, Some('n'), err),
                    'e' => match remain {
                        Some(dir) => {
                            if dir == 's' {
                                (x + 1, y - 1, None, err)
                            } else if dir == 'n' {
                                (x + 1, y + 1, None, err)
                            } else {
                                (x + 1, y, None, Some(AocError::ParsingError))
                            }
                        }
                        None => (x + 1, y, None, err),
                    },
                    'w' => match remain {
                        Some(dir) => {
                            if dir == 's' {
                                (x - 1, y - 1, None, err)
                            } else if dir == 'n' {
                                (x - 1, y + 1, None, err)
                            } else {
                                (x - 1, y, None, Some(AocError::ParsingError))
                            }
                        }
                        None => (x - 1, y, None, err),
                    },
                    _ => (0, 0, None, Some(AocError::ParsingError)),
                });
        if remain.is_some() || res.is_some() {
            // println!("Error: {:?} {:?}", remain, res);
            Err(AocError::ParsingError)
        } else {
            // println!("Ok");
            Ok(Position { x, y })
        }
    }
}
pub fn main() {
    let values: Vec<Position> = read_lines_to_vec_t("day_2020_24.data");
    println!("{:?}", &values);
    println!("Part 1: {:?}", part_1(&values));
    println!("Return to 0: {:?}", "nwwswee".parse::<Position>());
    //println!("Part 2: {:?}", process(&values, 3, 2020));
}

#[must_use]
fn part_1(positions: &[Position]) -> usize {
    let mut hmap = HashMap::new();
    for v in positions.iter().map(|Position { x, y }| (*x, *y)) {
        *hmap.entry(v).or_insert(0_usize) += 1;
    }
    hmap.iter()
        .inspect(|h| println!("{:?}", h))
        .filter(|&(_, val)| *val % 2 != 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part_1() {
        let values = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<Position>>();
        assert_eq!(part_1(&values), 10);
    }
}
