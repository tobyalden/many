use macroquad::prelude::*;

pub struct Room {}

impl Room {
    //pub fn new(num_players: usize) -> Self {
    //assert!(num_players <= 4);
    //Self {
    //num_players,
    //game_state: State::new(num_players),
    //local_handles: Vec::new(),
    //last_checksum: (NULL_FRAME, 0),
    //periodic_checksum: (NULL_FRAME, 0),
    //}
    //}

    // renders the game to the window
    pub fn render(&self) {
        clear_background(BLUE);

        draw_text(
            "IN ROOM. Press ENTER to join game.",
            20.0,
            40.0,
            30.0,
            WHITE,
        );
    }
}
