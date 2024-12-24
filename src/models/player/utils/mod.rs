use crate::dto::result::ResultDTO;
use crate::{COLUMN_MAX, COLUMN_MIN, ROW_MAX, ROW_MIN};

pub fn is_quit_instruction(input: &str) -> bool {
    input.eq_ignore_ascii_case("q")
}

pub fn try_get_coordinates(input: &str) -> ResultDTO<(u8, u8)> {
    if input.is_empty() {
        return ResultDTO::create_failed_result("Invalid data provided by user.");
    }
    let splitted_input: Vec<&str> = input.trim().split(' ').collect();
    if splitted_input.len() != 2 {
        return ResultDTO::create_failed_result("Invalid data provided by user.");
    } else {
        let mut coordinates: Vec<u8> = vec![];
        for i in 0..splitted_input.len() {
            let result = match splitted_input[i].parse::<u8>() {
                Ok(num) => num,
                Err(_) => {
                    return ResultDTO::create_failed_result("ParseError");
                }
            };
            coordinates.push(result);
        }
        return ResultDTO::create_success_result(Some((coordinates[0], coordinates[1])));
    }
}

pub fn ensure_row_coordinates_valid(coordinate: u8) -> bool {
    coordinate >= ROW_MIN && coordinate <= ROW_MAX
}

pub fn ensure_column_coordinates_valid(coordinate: u8) -> bool {
    coordinate >= COLUMN_MIN && coordinate <= COLUMN_MAX
}
