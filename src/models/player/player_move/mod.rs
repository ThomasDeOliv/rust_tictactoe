pub struct PlayerMove {
    row: u8,
    column: u8,
}

impl PlayerMove {
    pub fn new(row: u8, column: u8) -> Self {
        PlayerMove { row, column }
    }

    pub fn get_played_row(&self) -> u8 {
        self.row
    }

    pub fn get_played_column(&self) -> u8 {
        self.column
    }
}
