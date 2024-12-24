use std::thread;
use std::time::Duration;

use super::player_move::PlayerMove;
use crate::dto::result::ResultDTO;
use rand::Rng;

pub fn ai_play(empty_cells: Vec<(u8, u8, char)>) -> ResultDTO<PlayerMove> {
    let mut rng = rand::thread_rng();
    let rnd_index: usize = rng.gen_range(0..empty_cells.len());
    thread::sleep(Duration::from_millis(2000));
    return ResultDTO::create_success_result(Some(PlayerMove::new(
        empty_cells[rnd_index].0,
        empty_cells[rnd_index].1,
    )));
}
