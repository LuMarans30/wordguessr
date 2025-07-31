use crate::{
    model::{dialog::Dialog, grid::Grid},
    service::dictionary::Word,
};

#[derive(Clone)]
pub struct GameState {
    pub grid: Grid,
    pub secret_word: Word,
    pub word_length: usize,
    pub num_tries: usize,
    pub status: GameStatus,
    pub current_dialog: Option<Dialog>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
}

impl GameState {
    pub fn new(secret_word: Word, num_tries: usize, word_length: usize) -> Self {
        Self {
            grid: Grid::new(num_tries, word_length),
            secret_word,
            word_length,
            num_tries,
            status: GameStatus::Playing,
            current_dialog: None,
        }
    }

    pub fn is_game_over(&self) -> bool {
        matches!(self.status, GameStatus::Won | GameStatus::Lost)
    }
}
