mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    cells: Vec<bool>,
    rows: usize,
    cols: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cells: Vec<bool> = Vec::with_capacity(rows * cols);
        for i in 0..rows * cols {
            cells[i] = rand::random();
        }

        Self { rows, cols, cells }
    }

    pub fn tick(&mut self) {
        for index in 0..self.cells.len() {
            let live_neighbor_count = self.number_of_live_neighbors(index);
            let cell = &mut self.cells[index];

            match (&cell, live_neighbor_count) {
                (false, 3) => *cell = true,
                (true, x) if (x < 2 || x > 3) => *cell = false,
                (true, _) => {}
                _ => {}
            }
        }
    }

    fn number_of_live_neighbors(&self, index: usize) -> usize {
        let (row, col) = self.get_row_col(index);

        let prev_row = (row + self.rows - 1) % self.rows;
        let next_row = (row + 1) % self.rows;
        let prev_col = (col + self.cols - 1) % self.cols;
        let next_col = (col + 1) % self.cols;

        let mut live_count = 0;
        for (row, col) in [
            (prev_row, prev_col),
            (prev_row, col),
            (prev_row, next_col),
            (row, next_col),
            (next_row, next_col),
            (next_row, col),
            (next_row, prev_col),
        ] {
            let index = self.get_index((row, col));
            if self.cells[index] {
                live_count += 1;
            }
        }

        live_count
    }

    fn get_row_col(&self, index: usize) -> (usize, usize) {
        let row = index / self.cols;
        let col = index % self.cols;

        (row, col)
    }

    fn get_index(&self, (row, col): (usize, usize)) -> usize {
        row * self.cols + col
    }
}
