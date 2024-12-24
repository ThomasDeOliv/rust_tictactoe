use std::io::{stdin, stdout, Write};

pub fn get_user_input() -> String {
    let mut user_buffer = String::new();
    print!("Saisissez les coordonnées de la case que vous souhaitez jouer : ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_buffer)
        .expect("Did not enter a correct string");
    if let Some('\n') = user_buffer.chars().next_back() {
        user_buffer.pop();
    }
    if let Some('\r') = user_buffer.chars().next_back() {
        user_buffer.pop();
    }
    return user_buffer;
}
