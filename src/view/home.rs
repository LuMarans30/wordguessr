use maud::{Markup, html};

use crate::model::grid::Grid;

pub async fn root(grid: &Grid) -> Markup {
    html! {
        div style="display: flex; justify-content: center; width: 100%;" {
            div #grid-container .container .center-align style="max-width: 500px; margin: auto; padding: 1rem;" {
                (grid)
            }
        }
        // Placeholder for dialogs
        div #dialog-container {

        }
        div .padding .absolute .bottom .right {
            button ws-send="reset" hx-vals="{\"reset\": \"reset\"}" hx-swap="innerHTML" hx-target="#grid-container" .extend .square .round  {
                i { "replay" }
                span { "Replay" }
            }
        }
    }
}
