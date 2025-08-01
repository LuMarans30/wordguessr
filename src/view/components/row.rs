use maud::{Markup, Render, html};

use crate::model::row::Row;

impl Render for Row {
    fn render(&self) -> Markup {
        html! {
            div .row style="display: flex; gap: 4px; justify-content: center;" {
                @for cell in &self.cells {
                    (cell)
                }

                button ws-send="input" disabled[self.is_disabled()] .transparent .circle hx-include="input" hx-swap-oob="true" hx-target="grid-container" {
                    i { "arrow_forward" }
                }
            }
        }
    }
}
