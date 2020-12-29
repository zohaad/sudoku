use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use js_sys::Array;
use crate::cell::Cell;

pub struct Sudoku {
    matrix: [[Cell; 9]; 9],
    hints: VecDeque<Hint>,
}

// denotes row, column and value
pub struct Hint {
    i: usize,
    j: usize,
    value: usize,
}

impl Hint {
    fn new(i: usize, j: usize, value: usize) -> Self {
        // TODO: useless code? can delete?
        if !(1..=9).contains(&value) {
            panic!("this isn't allowed to happen");
        }
        Self { i, j, value }
    }
}

impl Sudoku {
    pub fn new(cells: [Cell; 81]) -> Self {
        // TODO: remove useless allocation
        let mut matrix: [[Cell; 9]; 9] = [[Cell::NoSolution; 9]; 9]; 
        let mut hints = VecDeque::new(); 

        for i in 0..9 {
            for j in 0..9 {
                matrix[i][j] = cells[i * 9 + j];
                if let Cell::Solution(value) = matrix[i][j] {
                    hints.push_back(Hint::new(i, j, value));
                }
            }
        }
        Self { matrix, hints }
    }

    pub fn test_new(hints: Vec<[usize; 3]>) -> Self {
        let mut matrix = [[Cell::Candidates([true; 9]); 9]; 9];
        let mut hints_deque = VecDeque::new();

        for [i, j, value] in hints {
            matrix[i][j] = Cell::Solution(value);
            hints_deque.push_back(Hint::new(i, j, value));
        }

        Self { matrix, hints: hints_deque }
    }

    // TODO: implement solve method
    pub fn solve(&mut self) {
        self.matrix = [[Cell::NoSolution; 9]; 9];
    }

    // helper method for DOM display
    pub fn ith_as_str(&self, i: usize) -> &'static str  {
        self.matrix[i / 9][i % 9].as_str()
    }
}

// to make sudoku.into() work for console.log
impl From<Sudoku> for JsValue {
    #[inline] // what does #[inline] even do?
    fn from(sudoku: Sudoku) -> Self {
        JsValue::from(
            sudoku.matrix.iter()
                .map(|row| row.into_iter()
                    .map(|c| JsValue::from(c.as_str()))
                    .collect::<Array>()
                )
                .collect::<Array>()
        )
    }
}