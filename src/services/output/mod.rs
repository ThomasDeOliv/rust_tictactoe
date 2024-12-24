use crate::models::board::Board;
use crate::models::player::Player;

pub fn display_line(separator: char) -> () {
    let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
    let line: String = std::iter::repeat(separator).take(width).collect();
    println!("{}", line);
}

pub fn display_board(board: &Board) -> () {
    display_line('-');
    println!("RUST Morpion");
    display_line('-');

    println!("|-----------------|");
    for i in 1..=3 {
        for j in 1..=3 {
            let cell_result = board.get_cell(i, j);
            if let Some(cell) = cell_result {
                print!("|  {0}  ", cell.2);
            }
        }
        print!("|\n");
        if i != 3 {
            println!("|-----|-----|-----|");
        }
    }
    println!("|-----------------|");
}

pub fn display_start_sentence(current_player: &Player) -> () {
    println!(
        "Player {0} - Enter row (1-3) and column (1-3), separated by a space, or 'q' to quit... ",
        current_player.get_symbol()
    );
}

pub fn display_already_played() -> () {
    println!("This game has already been played.");
}

pub fn display_already_played_cell() -> () {
    println!("This cell has already been played.");
}

pub fn display_draw() -> () {
    println!("It's a draw.");
}

pub fn display_unvalid_move() -> () {
    println!("It's an invalid move.");
}

pub fn display_won(player: &Player) -> () {
    println!("Player {} has won the game !", player.get_symbol());
}

pub fn display_clear() -> () {
    clearscreen::clear().expect("Failed to clear screen");
}
