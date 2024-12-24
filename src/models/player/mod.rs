pub mod ai_play;
pub mod allowed_char;
pub mod human_player;
pub mod player_move;
pub mod utils;

use self::ai_play::ai_play;
use self::allowed_char::AllowedChar;
use self::human_player::human_play;
use self::player_move::PlayerMove;
use crate::dto::result::ResultDTO;
use std::thread::spawn;

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

    pub fn get_next_move_async(&self, empty_cells: Vec<(u8, u8, char)>) -> ResultDTO<PlayerMove> {
        let is_ai = self.get_is_ai();
        let handle = spawn(move || {
            if !is_ai {
                human_play(empty_cells)
            } else {
                ai_play(empty_cells)
            }
        });
        match handle.join() {
            Ok(result) => result,
            Err(_) => ResultDTO::create_failed_result("CannotExecuteMove"),
        }
    }
}
