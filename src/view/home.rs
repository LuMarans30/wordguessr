use maud::{Markup, html};

use crate::model::grid::Grid;

pub async fn root(grid: &Grid) -> Markup {
    html! {
        // Placeholder for grid
        div #grid-container style="display: flex; flex-direction: column; justify-content: center; width: 100%;" {
            (grid)
            // Placeholder for message
            div #message-container {

            }
        }
        // Placeholder for dialog
        div #dialog-container {

        }
        div .padding .absolute .bottom .right {
            button ws-send="reset" hx-vals="{\"reset\": \"reset\"}" hx-swap-oob="true" hx-target="grid-container" .extend .square .round  {
                i { "replay" }
                span { "Replay" }
            }
        }
    }
}
