use std::sync::Arc;

use color_eyre::Result;
use maud::{Markup, html};
use tokio::sync::RwLock;

use crate::{
    controller::game_controller::GameController,
    model::game_state::{GameState, GameStatus},
    service::dictionary::Word,
};

pub struct InputController {
    game_controller: Arc<GameController>,
}

impl InputController {
    pub fn new(game_controller: Arc<GameController>) -> Self {
        Self { game_controller }
    }

    pub async fn handle_input(
        &self,
        game_state: &Arc<RwLock<GameState>>,
        input: Vec<char>,
    ) -> Result<Markup> {
        let mut state = game_state.write().await;

        match self.game_controller.process_guess(&mut state, input).await {
            Ok(_) => Ok(self.render_game_state(&state)),
            Err(_) => Ok(self.render_error()),
        }
    }

    pub fn render_game_state(&self, state: &GameState) -> Markup {
        html! {
            div {
                (state.grid)
                br;
                div .medium-line {
                    @if let GameStatus::Won | GameStatus::Lost = state.status {
                        (self.render_game_over_message(&state.status, &state.secret_word))
                    }
                }
            }
            @if let Some(dialog) = &state.current_dialog {
                (dialog)
            }
        }
    }

    fn render_game_over_message(&self, status: &GameStatus, word: &Word) -> Markup {
        let status_text = match status {
            GameStatus::Won => "You've won!",
            GameStatus::Lost => "You've lost!",
            _ => unreachable!(),
        };

        html! {
            (html! {
                h5 { (status_text) }
                br;
                p { "The secret word is: "(word) }
                br;
                (self.render_meanings_list(word))
            })
        }
    }

    fn render_meanings_list(&self, word: &Word) -> Markup {
        if word.metadata.meanings.is_empty() {
            return html! {};
        }

        html! {
            p {
                {"Definitions of "(word.word)": "}
                br;
                ul {
                    @for meaning in &word.metadata.meanings {
                        li { (meaning) }
                    }
                }
            }
        }
    }

    fn render_error(&self) -> Markup {
        html! {
            div .error {
                "An error occurred. Please try again."
            }
        }
    }
}
