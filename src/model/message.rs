use crate::{model::game_state::GameStatus, service::dictionary::Word};

#[derive(Clone, Debug)]
pub struct Message {
    pub status: GameStatus,
    pub secret_word: Word,
}

impl Message {
    pub fn new(status: GameStatus, secret_word: Word) -> Self {
        Self {
            status,
            secret_word,
        }
    }
}
