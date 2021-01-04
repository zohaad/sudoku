use sudoku::Sudoku;
// use std::str::FromStr;
// use std::convert::TryInto;

// #[cfg(test)]
fn get_sudoku_from_file(path: &str) -> Sudoku {
    // let hints: Vec<[usize; 3]> = std::fs::read_to_string(path).unwrap()
    //     .lines()
    //     .map(|line| line.split_ascii_whitespace().map(|n | usize::from_str(n).unwrap()).collect::<Vec<usize>>().try_into().unwrap())
    //     .collect();

    let string = std::fs::read_to_string(path).unwrap();

    let hints: Vec<[usize; 3]> = string.lines().map(|line| {
        let row = line[..1].parse().unwrap();
        let col = line[2..3].parse().unwrap();
        let value = line[4..5].parse().unwrap();
        [row, col, value]
    }).collect();

    Sudoku::test_new(hints)
}

// cargo test -- --nocapture

// or
// doesn't work here
// rustc --test main.rs; ./main

// or (my favorite)
// doesn't work
// cargo watch “test -- --nocapture”

#[test]
fn sudoku_1() {
    let mut sudoku = get_sudoku_from_file("./tests/resources/sudoku1.txt");
    let answer = get_sudoku_from_file("./tests/resources/answer1.txt");
    // println!("{:?}", sudoku);
    sudoku.solve();
    assert_eq!(sudoku, answer);
}
// #[test]
fn sudoku_2() {
    let mut sudoku = get_sudoku_from_file("./tests/resources/sudoku2.txt");
    let answer = get_sudoku_from_file("./tests/resources/answer2.txt");
    sudoku.solve();
    assert_eq!(sudoku, answer)
}
// #[test]
// fn sudoku_3() {
//     let sudoku = get_sudoku_from_file("./tests/resources/sudoku3.txt");
//     let answer = get_sudoku_from_file("./tests/resources/answer3.txt");
//     assert_eq!(sudoku, answer)
// }
// #[test]
// fn sudoku_4() {
//     let sudoku = get_sudoku_from_file("./tests/resources/sudoku4.txt");
//     let answer = get_sudoku_from_file("./tests/resources/answer4.txt");
//     assert_eq!(sudoku, answer)
// }
// #[test]
// fn sudoku_5() {
//     let sudoku = get_sudoku_from_file("./tests/resources/sudoku5.txt");
//     let answer = get_sudoku_from_file("./tests/resources/answer5.txt");
//     assert_eq!(sudoku, answer)
// }
// #[test]
// fn sudoku_6() {
//     let sudoku = get_sudoku_from_file("./tests/resources/sudoku6.txt");
//     let answer = get_sudoku_from_file("./tests/resources/answer6.txt");
//     assert_eq!(sudoku, answer)
// }
