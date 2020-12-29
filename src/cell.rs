#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Candidates([bool; 9]),
    Solution(usize),
    NoSolution,
}


impl Cell {
    pub fn as_str(&self) -> &'static str {
        match self {
            Cell::Candidates(_) => "?",
            Cell::NoSolution => "x",
            Cell::Solution(1) => "1",
            Cell::Solution(2) => "2",
            Cell::Solution(3) => "3",
            Cell::Solution(4) => "4",
            Cell::Solution(5) => "5",
            Cell::Solution(6) => "6",
            Cell::Solution(7) => "7",
            Cell::Solution(8) => "8",
            Cell::Solution(9) => "9",
            _ => "y" // should never happen
        }
    }
}