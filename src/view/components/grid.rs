use maud::{Markup, Render, html};

use crate::model::grid::Grid;

impl Render for Grid {
    fn render(&self) -> Markup {
        html! {
            div id="grid-container" .container .center-align style="max-width: 500px; margin: auto; padding: 1rem;" {
                @for row in &self.rows {
                    (row)
                }
            }
        }
    }
}
