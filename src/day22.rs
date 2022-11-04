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

    // let player_1: [usize; 5] = [9, 2, 6, 3, 1];
    // let player_2: [usize; 5] = [5, 8, 4, 7, 10];

    let mut p1: VecDeque<usize> = player_1.into();
    let mut p2: VecDeque<usize> = player_2.into();

    let mut turn = 0;
    loop {
        // println!("Turn: {}", turn);
        play_turn(&mut p1, &mut p2);
        if p1.is_empty() || p2.is_empty() {
            break;
        }
        turn += 1;
        // println!();
    }

    // println!("p1: {:?}", p1);
    // println!("p2: {:?}", p2);
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
    // println!("Player 1 deck: {:?}", p1);
    // println!("Player 2 deck: {:?}", p2);
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    // println!("Player 1: {}, Player 2: {}", c1, c2);
    if c1 > c2 {
        // println!("Player 1 wins, new deck: {:?}", p1);
        p1.push_back(c1);
        p1.push_back(c2);
    }
    if c1 < c2 {
        // println!("Player 2 wins, new deck: {:?}", p2);
        p2.push_back(c2);
        p2.push_back(c1);
    }
    if c1 == c2 {
        panic!("Don't know what to do!");
    }
}
