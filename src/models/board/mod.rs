pub mod utils;

use self::utils::{group_by_columns, group_by_diagonals, group_by_rows};
use crate::{COLUMN_MAX, COLUMN_MIN, EMPTY_CHAR, ROW_MAX, ROW_MIN};

pub struct Board {
    cells: Vec<(u8, u8, char)>,
}

impl Board {
    pub fn new() -> Self {
        let mut cells: Vec<(u8, u8, char)> = vec![];
        for i in ROW_MIN..=ROW_MAX {
            for j in COLUMN_MIN..=COLUMN_MAX {
                cells.push((i, j, EMPTY_CHAR));
            }
        }
        Board { cells }
    }

    pub fn get_cell(&self, row: u8, column: u8) -> Option<(u8, u8, char)> {
        if row >= ROW_MIN && row <= ROW_MAX && column >= COLUMN_MIN && column <= COLUMN_MAX {
            let index = (COLUMN_MAX * (row - 1) + (column - 1)) as usize;
            if let Some(target_cell) = self.cells.get(index) {
                return Some((target_cell.0, target_cell.1, target_cell.2));
            }
        }
        None
    }

    pub fn get_empty_cells(&self) -> Vec<(u8, u8, char)> {
        self.cells
            .iter()
            .filter(|cell| cell.2 == EMPTY_CHAR)
            .map(|cell| *cell)
            .collect()
    }

    pub fn update_cell(&mut self, row: u8, column: u8, value: char) -> () {
        let cell_result = self.get_cell(row, column);
        if let Some(cell) = cell_result {
            if cell.2 == EMPTY_CHAR {
                let index = (COLUMN_MAX * (row - 1) + (column - 1)) as usize;
                self.cells[index].2 = value;
            }
        }
    }

    pub fn is_board_win(&self) -> bool {
        let cells_by_rows = group_by_rows(&self);
        if cells_by_rows
            .values()
            .any(|row| row.iter().all(|cell| cell.2 == 'X') || row.iter().all(|cell| cell.2 == 'O'))
        {
            return true;
        }

        let cells_by_columns = group_by_columns(&self);
        if cells_by_columns.values().any(|column| {
            column.iter().all(|cell| cell.2 == 'X') || column.iter().all(|cell| cell.2 == 'O')
        }) {
            return true;
        }

        let cells_by_diagonals = group_by_diagonals(&self);
        if cells_by_diagonals.values().any(|diagonal| {
            diagonal.iter().all(|cell| cell.2 == 'X') || diagonal.iter().all(|cell| cell.2 == 'O')
        }) {
            return true;
        }

        return false;
    }

    pub fn is_board_full(&self) -> bool {
        let mut is_full: bool = true;
        for cell in &self.cells {
            if cell.2 == EMPTY_CHAR {
                is_full = false;
                break;
            }
        }
        return is_full;
    }
}
