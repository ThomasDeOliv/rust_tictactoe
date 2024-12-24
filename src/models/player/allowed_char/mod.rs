pub enum AllowedChar {
    X,
    O,
}

impl PartialEq for AllowedChar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AllowedChar::X, AllowedChar::X) | (AllowedChar::O, AllowedChar::O) => true,
            _ => false,
        }
    }
}
