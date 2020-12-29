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
    let mut hints: Vec<[usize; 3]> = Vec::new();

    for line in string.lines() {
        let row = line[..1].parse().unwrap();
        let col = line[2..3].parse().unwrap();
        let value = line[4..5].parse().unwrap();

        hints.push([row, col, value]);
    }
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
fn it_works() {
    get_sudoku_from_file("./tests/resources/sudoku1.txt");
    assert_eq!(2 + 2, 4);
}