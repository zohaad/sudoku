use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

mod cell;
use crate::cell::Cell;

mod sudoku;
use crate::sudoku::Sudoku;

#[wasm_bindgen(start)]
pub fn run() ->  Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let mut cells: [Cell; 81] = [Cell::Candidates([true; 9]); 81];
    let mut solution_count = 0;

    // get from DOM
    for i in 0..81 {
        let el = document.get_element_by_id(&i.to_string()).unwrap();

        match el.dyn_into::<HtmlInputElement>().unwrap().value().parse::<usize>() {
            Ok(number) if (1..=9).contains(&number) => {
                solution_count += 1;
                cells[i] = Cell::Solution(number);
            },
            _ => {}
        }
    }

    if solution_count < 17 {
        console::error_1(&"Solution count is below 17".into());
        panic!(); // stop execution
    }
    
    let mut sudoku = Sudoku::new(cells);
    sudoku.solve();

    // put in DOM
    for i in 0..81 {
        let el = document.get_element_by_id(&i.to_string()).unwrap();
        el.dyn_into::<HtmlInputElement>().unwrap().set_value(sudoku.ith_as_str(i));
    }

    Ok(())
}

// to run tests
#[cfg(test)]
mod tests;
