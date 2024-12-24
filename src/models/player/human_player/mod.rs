use super::player_move::PlayerMove;
use super::utils::{
    ensure_column_coordinates_valid, ensure_row_coordinates_valid, is_quit_instruction,
    try_get_coordinates,
};
use crate::dto::result::ResultDTO;
use crate::services::input::get_user_input;
use crate::EMPTY_CHAR;

pub fn human_play(empty_cells: Vec<(u8, u8, char)>) -> ResultDTO<PlayerMove> {
    let input: String = get_user_input();
    if input.is_empty() {
        return ResultDTO::create_failed_result("ProvidedEmptyInput");
    } else if is_quit_instruction(&input) {
        return ResultDTO::create_failed_result("Quit");
    } else {
        if let Some(result) = try_get_coordinates(&input) {
            if !ensure_row_coordinates_valid(result.0) || !ensure_column_coordinates_valid(result.1)
            {
                return ResultDTO::create_failed_result("InvalidCoordinates");
            } else if let Some(cell) = empty_cells
                .iter()
                .find(|cell| cell.0 == result.0 && cell.1 == result.1 && cell.2 == EMPTY_CHAR)
            {
                return ResultDTO::create_success_result(Some(PlayerMove::new(cell.0, cell.1)));
            } else {
                return ResultDTO::create_failed_result("CellNotAvailable");
            }
        } else {
            return ResultDTO::create_failed_result("ParseStringError");
        }
    }
}
