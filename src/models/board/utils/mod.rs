use super::Board;
use std::collections::HashMap;

pub fn group_by_rows(board: &Board) -> HashMap<u8, Vec<(u8, u8, char)>> {
    let mut grouped_cells: HashMap<u8, Vec<(u8, u8, char)>> = HashMap::new();
    for cell in &board.cells {
        grouped_cells
            .entry(cell.0)
            .or_insert_with(Vec::new)
            .push((cell.0, cell.1, cell.2));
    }
    grouped_cells
}

pub fn group_by_columns(board: &Board) -> HashMap<u8, Vec<(u8, u8, char)>> {
    let mut grouped_cells: HashMap<u8, Vec<(u8, u8, char)>> = HashMap::new();
    for cell in &board.cells {
        grouped_cells
            .entry(cell.0)
            .or_insert_with(Vec::new)
            .push((cell.0, cell.1, cell.2));
    }
    grouped_cells
}

pub fn group_by_diagonals(board: &Board) -> HashMap<u8, Vec<(u8, u8, char)>> {
    let first_diag_cells: Vec<(u8, u8, char)> = board
        .cells
        .iter()
        .filter(|cell| cell.0 == cell.1)
        .map(|cell| *cell)
        .collect();
    let second_diag_cells: Vec<(u8, u8, char)> = board
        .cells
        .iter()
        .filter(|cell| cell.0 + cell.1 == 4)
        .map(|cell| *cell)
        .collect();
    let mut diagonals_map: HashMap<u8, Vec<(u8, u8, char)>> = HashMap::new();
    diagonals_map.insert(1, first_diag_cells);
    diagonals_map.insert(2, second_diag_cells);
    diagonals_map
}
