use super::board::Board;
use super::player::allowed_char::AllowedChar;
use super::player::player_move::PlayerMove;
use super::player::Player;
use crate::dto::result::ResultDTO;
use crate::services::input::get_user_choice;
use crate::services::output::{
    display_already_played, display_already_played_cell, display_board, display_clear,
    display_draw, display_errors_handled, display_init_game, display_invalid, display_quit,
    display_start_sentence, display_unvalid_move, display_won,
};
use crate::EMPTY_CHAR;

pub struct Game {
    board: Board,
    played: bool,
    players: Vec<Player>,
    current_player_index: usize,
}

impl Game {
    pub fn new() -> Self {
        let board: Board = Board::new();
        let played: bool = false;
        let players: Vec<Player> = vec![];
        let current_player_index: usize = 0;
        Game {
            board,
            played,
            players,
            current_player_index,
        }
    }

    pub fn change_player(&mut self) -> () {
        if self.current_player_index == 0 {
            self.current_player_index = 1;
        } else {
            self.current_player_index = 0;
        }
    }

    pub fn get_was_played(&self) -> bool {
        self.played
    }

    pub fn close_game(&mut self) -> () {
        self.played = true;
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_index as usize]
    }

    pub fn init(&mut self) -> () {
        display_init_game();
        let choice = get_user_choice();
        match choice.as_str() {
            "1" => {
                self.players.push(Player::new(false, AllowedChar::X));
                self.players.push(Player::new(false, AllowedChar::O));
            }
            "2" => {
                self.players.push(Player::new(false, AllowedChar::O));
                self.players.push(Player::new(false, AllowedChar::X));
            }
            "3" => {
                self.players.push(Player::new(true, AllowedChar::X));
                self.players.push(Player::new(false, AllowedChar::O));
            }
            "4" => {
                self.players.push(Player::new(false, AllowedChar::X));
                self.players.push(Player::new(true, AllowedChar::O));
            }
            "5" => {
                self.players.push(Player::new(true, AllowedChar::X));
                self.players.push(Player::new(true, AllowedChar::O));
            }
            "q" => {
                display_quit();
                std::process::exit(0);
            }
            _ => {
                display_invalid();
                std::process::exit(0);
            }
        };
        display_clear();
    }

    pub fn play(&mut self) -> () {
        if self.played {
            display_clear();
            display_already_played();
        } else {
            display_board(&self.board);
            loop {
                let current_player = &self.players[self.current_player_index];
                display_start_sentence(current_player);
                let player_move_result: ResultDTO<PlayerMove> =
                    current_player.get_next_move_async(self.board.get_empty_cells());
                if let Some(player_move) = player_move_result.get_result() {
                    if let Some(target_cell) = self.board.get_cell(
                        player_move.get_played_row(),
                        player_move.get_played_column(),
                    ) {
                        if target_cell.2 == EMPTY_CHAR {
                            self.board.update_cell(
                                target_cell.0,
                                target_cell.1,
                                self.players[self.current_player_index].get_symbol(),
                            );
                            display_clear();
                            display_board(&self.board);
                            if self.board.is_board_win() {
                                display_won(&self.players[self.current_player_index]);
                                break;
                            }
                            if self.board.is_board_full() {
                                display_draw();
                                break;
                            }
                            self.change_player();
                        } else {
                            display_clear();
                            display_board(&self.board);
                            display_already_played_cell();
                        }
                    } else {
                        display_clear();
                        display_board(&self.board);
                        display_unvalid_move();
                    }
                } else {
                    display_clear();
                    display_board(&self.board);
                    display_errors_handled(player_move_result.get_reason());
                }
            }
            self.close_game();
        }
    }
}
