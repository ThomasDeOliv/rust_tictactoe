use super::board::Board;
use super::player::allowed_char::AllowedChar;
use super::player::player_move::PlayerMove;
use super::player::Player;
use crate::dto::result::ResultDTO;
use crate::services::output::{
    display_already_played, display_already_played_cell, display_board, display_clear,
    display_draw, display_start_sentence, display_unvalid_move, display_won,
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
        let players: Vec<Player> = vec![
            Player::new(false, AllowedChar::X),
            Player::new(false, AllowedChar::O),
        ];
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
                    current_player.get_next_move_async();
                if player_move_result.is_success() {
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
                        display_unvalid_move();
                    }
                } else {
                    display_clear();
                    display_board(&self.board);
                    display_unvalid_move();
                }
            }
            self.close_game();
        }
    }
}
