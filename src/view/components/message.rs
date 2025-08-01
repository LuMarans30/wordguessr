use maud::{Markup, Render, html};

use crate::model::{game_state::GameStatus, message::Message};

impl Render for Message {
    fn render(&self) -> Markup {
        html!(
            div hx-swap-oob="innerHTML:#message-container" {
                div .medium-line {
                    @if let GameStatus::Won | GameStatus::Lost = self.status {
                        @if self.status == GameStatus::Won {
                            h5 {"You've won"}
                        } @else {
                            h5 {"You've lost!"}
                            br;
                            p {"The secret word is: "(self.secret_word)}
                        }
                        br;
                        p {
                            {"Definitions of "(self.secret_word.word)": "}
                            br;
                            ul {
                                @for meaning in &self.secret_word.meanings {
                                    li { (meaning) }
                                }
                            }
                        }
                    }
                }
            }
        )
    }
}
