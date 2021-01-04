use std::collections::VecDeque;
use wasm_bindgen::JsValue;
use js_sys::Array;
use crate::cell::Cell;
use std::fmt::{ self, Formatter, Debug };
use std::cmp::PartialEq;

pub struct Sudoku {
    matrix: [[Cell; 9]; 9],
    hints: VecDeque<Hint>,
}

// denotes row, column and value
#[derive(Debug)]
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
        self.simple_logic_solve()

    }

    fn simple_logic_solve(&mut self) {
        while let Some(hint) = self.hints.pop_front() {
            let Hint {i: h_i, j: h_j, value} = hint; 
            for i in (0..9).filter(|&x|  x != h_i) {
                self.remove(i, h_j, value);
            }

            for j in (0..9).filter(|&x| x != h_j) {
                self.remove(h_i, j, value);
            }

            let start_row = (h_i / 3) * 3;
            let end_row = start_row + 3;
            let start_col = (h_j / 3) * 3;
            let end_col = start_col + 3;

            for i in start_row..end_row {
                for j in start_col..end_col {
                    if i != h_i && j != h_j {
                        self.remove(i, j, value);
                    }
                }
            }
        }
    }

    fn remove(&mut self, i: usize, j: usize, value: usize) {
        match self.matrix[i][j] {
            Cell::Candidates(ref mut array) => {
                array[value - 1] = false;
                let mut num_trues = 0;
                let mut idx = 0;
                for (x, &v) in array.iter().enumerate() {
                    if v {
                        num_trues += 1;
                        idx = x;
                    }
                }
                if num_trues == 1 { // solution found
                    self.matrix[i][j] = Cell::Solution(idx + 1);
                    self.hints.push_back(Hint{i, j, value: idx + 1});
                }
            },
            Cell::Solution(solution) => {
                if solution == value {
                    self.matrix[i][j] = Cell::NoSolution;
                }
            },
            Cell::NoSolution => {},
        }
    }

    fn solved(&self) -> bool {
        self.matrix.iter().flatten().all(|c| { matches!(c, Cell::Solution(_)) })
    }
    
    // helper method for DOM display
    pub fn ith_as_str(&self, i: usize) -> String  {
        self.matrix[i / 9][i % 9].to_string()
    }
}

// to make sudoku.into() work for console.log
impl From<Sudoku> for JsValue {
    #[inline] // what does #[inline] even do?
    fn from(sudoku: Sudoku) -> Self {
        JsValue::from(
            sudoku.matrix.iter()
                .map(|row| row.into_iter()
                    .map(|c| JsValue::from(c))
                    .collect::<Array>()
                )
                .collect::<Array>()
        )
    }
}

// to print it out as {:?}
impl Debug for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..9 {
            for j in 0..9 {
                self.matrix[i][j].fmt(f)?;
                write!(f, " ")?;
                
                if j == 2 || j == 5 {
                    write!(f, "| ")?;
                }
            }
                        
            if i < 8 {
                writeln!(f)?; // new line
            }
            
            if i == 2 || i == 5 {
                writeln!(f, "---------------------")?;
            }
        }
        Ok(())
    }
}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if !(self.matrix[i][j] == other.matrix[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}