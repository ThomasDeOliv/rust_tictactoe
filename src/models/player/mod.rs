pub mod allowed_char;
pub mod player_move;
pub mod utils;

use self::allowed_char::AllowedChar;
use self::player_move::PlayerMove;
use self::utils::{ensure_row_coordinates_valid, is_quit_instruction, try_get_coordinates};
use crate::dto::result::ResultDTO;
use crate::services::input::get_user_input;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub struct Player {
    is_ai: bool,
    symbol: char,
}

impl Player {
    pub fn new(is_ai: bool, symbol: AllowedChar) -> Self {
        Player {
            is_ai,
            symbol: if symbol == AllowedChar::O { 'O' } else { 'X' },
        }
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn get_is_ai(&self) -> bool {
        self.is_ai
    }

    pub fn get_next_move_async(&self) -> ResultDTO<PlayerMove> {
        let data = Arc::new(self.get_is_ai());
        let handle: JoinHandle<ResultDTO<PlayerMove>> = spawn(move || {
            if !*data {
                let input: String = get_user_input();
                if input.is_empty() {
                    return ResultDTO::create_failed_result("ProvidedEmptyInput");
                } else if is_quit_instruction(&input) {
                    return ResultDTO::create_failed_result("Quit");
                } else {
                    let coordinates = try_get_coordinates(&input);
                    if let Some(result) = coordinates.get_result() {
                        if !ensure_row_coordinates_valid(result.0) {
                            return ResultDTO::create_failed_result(
                                "InvalidProvidedRowCoordinates",
                            );
                        } else if !ensure_row_coordinates_valid(result.1) {
                            return ResultDTO::create_failed_result(
                                "InvalidProvidedColumnCoordinates",
                            );
                        } else {
                            return ResultDTO::create_success_result(Some(PlayerMove::new(
                                result.0, result.1,
                            )));
                        }
                    } else {
                        return ResultDTO::create_failed_result("InvalidProvidedData");
                    }
                }
            } else {
                return ResultDTO::create_failed_result("NotImplementedYet");
            }
        });
        if let Ok(result) = handle.join() {
            return result;
        } else {
            return ResultDTO::create_failed_result("CannotExecuteMove");
        }
    }
}
