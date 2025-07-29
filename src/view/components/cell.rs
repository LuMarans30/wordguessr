use maud::{Markup, Render, html};

use crate::model::cell::{Cell, CellState};

impl Render for Cell {
    fn render(&self) -> Markup {
        let cell_color: &'static str = match self.state {
            CellState::Correct => "green",
            CellState::Absent => "red",
            CellState::Present => "yellow",
            _ => "",
        };

        html! {
            div .field .border .small .fill {
                input
                    name="input[]"
                    type="text"
                    value=(self.to_string())
                    minlength="1"
                    maxlength="1"
                    required
                    disabled[self.is_disabled]
                    style={"flex: 1; max-width: 60px; text-align: center; background-color: "(cell_color)";"}
                    oninput="this.value = this.value.toUpperCase().replace(/[^a-z]/gi, '');";
            }
        }
    }
}
