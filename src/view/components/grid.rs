use maud::{Markup, Render, html};

use crate::model::grid::Grid;

impl Render for Grid {
    fn render(&self) -> Markup {
        html! {
            @for row in &self.rows {
                (row)
            }
        }
    }
}
