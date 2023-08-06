use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub struct Universe {
    cells: FixedBitSet,
    rows: usize,
    cols: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(rows: usize, cols: usize) -> Self {
        utils::set_panic_hook();
        let mut cells = FixedBitSet::with_capacity(rows * cols);
        for i in 0..rows * cols {
            cells.set(i, js_sys::Math::random() > 0.5);
        }

        Self { rows, cols, cells }
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        for index in 0..self.cells.len() {
            let live_neighbor_count = self.number_of_live_neighbors(index);
            let cell = &self.cells[index];

            match (cell, live_neighbor_count) {
                (false, 3) => new_cells.set(index, true),
                (true, x) if (x < 2 || x > 3) => new_cells.set(index, false),
                _ => {}
            }
        }

        self.cells = new_cells;
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
            (row, prev_col),
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
