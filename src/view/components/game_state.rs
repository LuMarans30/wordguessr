use maud::{Markup, Render, html};

use crate::model::{game_state::GameState, message::Message};

impl Render for GameState {
    fn render(&self) -> Markup {
        html! {
            div #grid-container .container .center-align style="max-width: 500px; margin: auto; padding: 1rem;" {
                (self.grid)
                br;
                div id="message-container" {
                    (Message::new(self.status.clone(), self.secret_word.clone()))
                }
            }
            @if let Some(dialog) = &self.current_dialog {
                (dialog)
            }
        }
    }
}
