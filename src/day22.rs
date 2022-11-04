use std::collections::VecDeque;
fn main() {
    let player_1: [usize; 25] = [
        2, 31, 14, 45, 33, 18, 29, 36, 44, 47, 38, 6, 9, 5, 48, 17, 50, 41, 4, 21, 42, 23, 25, 28,
        3,
    ];
    let player_2: [usize; 25] = [
        26, 16, 27, 12, 49, 32, 19, 46, 37, 15, 10, 30, 11, 24, 1, 40, 7, 8, 43, 34, 20, 35, 22,
        39, 13,
    ];

    let mut p1: VecDeque<usize> = player_1.into();
    let mut p2: VecDeque<usize> = player_2.into();

    loop {
        play_turn(&mut p1, &mut p2);
        if p1.is_empty() || p2.is_empty() {
            break;
        }
    }
    let p = if p1.is_empty() { p2 } else { p1 };
    let res: usize = p
        .iter()
        .rev()
        .zip(1..)
        .map(|(val, idx)| val * idx)
        .sum::<usize>();
    println!("Part 1: {}", res);
}

fn play_turn(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    match c1.cmp(&c2) {
        std::cmp::Ordering::Less => {
            p2.push_back(c2);
            p2.push_back(c1);
        }

        std::cmp::Ordering::Greater => {
            p1.push_back(c1);
            p1.push_back(c2);
        }
        std::cmp::Ordering::Equal => panic!("Don't know what to do!"),
    }
}
