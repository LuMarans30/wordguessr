use std::sync::Arc;

use color_eyre::Result;
use maud::{Markup, html};
use tokio::sync::RwLock;

use crate::{
    controller::game_controller::GameController,
    model::{game_state::GameState, message::Message},
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
        let result = self.game_controller.process_guess(&mut state, input).await;
        let message = Message::new(state.status.clone(), state.secret_word.clone());

        match result {
            Ok(_) => Ok(self.render_game_state(message, &state)),
            Err(_) => Ok(self.render_error()),
        }
    }

    pub fn render_game_state(&self, message: Message, state: &GameState) -> Markup {
        html! {
            div {
                (state.grid)
                br;
                (message)
            }
            @if let Some(dialog) = &state.current_dialog {
                (dialog)
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
