use std::fmt::{ self, Display, Formatter, Debug};
use wasm_bindgen::JsValue;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Candidates([bool; 9]),
    Solution(usize),
    NoSolution,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cell::Candidates(_) => String::from(" "),
            Cell::NoSolution => String::from("x"),
            Cell::Solution(1) => String::from("1️"),
            Cell::Solution(2) => String::from("2️"),
            Cell::Solution(3) => String::from("3️"),
            Cell::Solution(4) => String::from("4️"),
            Cell::Solution(5) => String::from("5️"),
            Cell::Solution(6) => String::from("6️"),
            Cell::Solution(7) => String::from("7️"),
            Cell::Solution(8) => String::from("8️"),
            Cell::Solution(9) => String::from("9️"),
            _ => String::from("y") // should never happen
        };
        write!(f, "{}", s)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cell::Candidates(array) => {
                array.iter().filter(|x| **x == true).count().to_string()
            },
            Cell::NoSolution => String::from("x"),
            Cell::Solution(1) => String::from("1️⃣"),
            Cell::Solution(2) => String::from("2️⃣"),
            Cell::Solution(3) => String::from("3️⃣"),
            Cell::Solution(4) => String::from("4️⃣"),
            Cell::Solution(5) => String::from("5️⃣"),
            Cell::Solution(6) => String::from("6️⃣"),
            Cell::Solution(7) => String::from("7️⃣"),
            Cell::Solution(8) => String::from("8️⃣"),
            Cell::Solution(9) => String::from("9️⃣"),
            _ => String::from("y") // should never happen
        };
        write!(f, "{}", s)
    }
}

impl From<&Cell> for JsValue {
    fn from(cell: &Cell) -> Self {
        Self::from(cell.to_string())
    }
}