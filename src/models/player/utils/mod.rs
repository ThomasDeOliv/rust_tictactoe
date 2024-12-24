use crate::{COLUMN_MAX, COLUMN_MIN, ROW_MAX, ROW_MIN};

pub fn is_quit_instruction(input: &str) -> bool {
    input.eq_ignore_ascii_case("q")
}

pub fn try_get_coordinates(input: &str) -> Option<(u8, u8)> {
    let splitted_input: Vec<&str> = input.trim().split(' ').collect();
    if splitted_input.len() == 2 {
        let mut coordinates: [u8; 2] = [0, 0];
        for i in 0..splitted_input.len() {
            let result = match splitted_input[i].parse::<u8>() {
                Ok(num) => num,
                Err(_) => {
                    return None;
                }
            };
            coordinates[i] = result;
        }
        let tuple = (coordinates[0], coordinates[1]);
        return Some(tuple);
    }
    None
}

pub fn ensure_row_coordinates_valid(coordinate: u8) -> bool {
    coordinate >= ROW_MIN && coordinate <= ROW_MAX
}

pub fn ensure_column_coordinates_valid(coordinate: u8) -> bool {
    coordinate >= COLUMN_MIN && coordinate <= COLUMN_MAX
}
