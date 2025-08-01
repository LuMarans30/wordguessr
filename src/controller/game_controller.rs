use std::sync::Arc;

use crate::{
    model::{
        cell::{Cell, CellState},
        game_state::{GameState, GameStatus},
        grid::GridError,
    },
    service::dictionary::{Word, WordService},
};
use color_eyre::Result;

pub struct GameController {
    word_service: Arc<dyn WordService>,
}

impl GameController {
    pub fn new(word_service: Arc<dyn WordService>) -> Self {
        Self { word_service }
    }

    pub async fn create_new_game(&self, num_tries: usize, word_length: usize) -> Result<GameState> {
        let secret_word = self.word_service.get_random_word(word_length).await?;
        Ok(GameState::new(secret_word, num_tries, word_length))
    }

    pub async fn process_guess(
        &self,
        game_state: &mut GameState,
        guess: Vec<char>,
    ) -> Result<GuessResult> {
        if game_state.is_game_over() {
            return Ok(GuessResult::GameAlreadyOver);
        }

        let guess_word: String = guess.iter().collect::<String>().to_ascii_uppercase();

        // Validate if the word exists in the dictionary
        if !self.word_service.validate_word(&guess_word).await? {
            return Ok(GuessResult::InvalidWord);
        }

        // Update grid with guess
        let current_row = game_state.grid.current_row;
        game_state.grid.rows[current_row].cells = self
            .determine_cell_states(&guess, &game_state.secret_word)
            .into_iter()
            .zip(guess.iter())
            .map(|(state, letter)| Cell::new(Some(*letter), false).with_state(state))
            .collect();

        // Check win condition
        if guess_word == game_state.secret_word.word {
            game_state.grid.rows[game_state.grid.current_row].set_disabled(true);
            game_state.status = GameStatus::Won;
            return Ok(GuessResult::Won);
        }

        // Try to advance to next row
        match game_state.grid.advance_row() {
            Ok(_) => Ok(GuessResult::Continue),
            Err(GridError::NoMoreRows) => {
                game_state.grid.rows[game_state.grid.current_row].set_disabled(true);
                game_state.status = GameStatus::Lost;
                Ok(GuessResult::Lost)
            }
        }
    }

    fn determine_cell_states(&self, guess: &[char], secret: &Word) -> Vec<CellState> {
        let mut secret: Vec<char> = secret.word.chars().collect();
        let mut states = vec![CellState::Absent; guess.len()];

        // Mark correct positions first
        for (i, (g, s)) in guess.iter().zip(secret.iter_mut()).enumerate() {
            if g == s {
                states[i] = CellState::Correct;
                *s = '\0';
            }
        }

        // Mark present letters
        for (i, g) in guess.iter().enumerate() {
            if states[i] == CellState::Absent
                && let Some(j) = secret.iter().position(|s| s == g)
            {
                states[i] = CellState::Present;
                secret[j] = '\0';
            }
        }
        states
    }
}

#[derive(Debug)]
pub enum GuessResult {
    Won,
    Lost,
    Continue,
    InvalidWord,
    GameAlreadyOver,
}
