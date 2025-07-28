use maud::{Markup, Render, html};

use crate::model::row::Row;

impl Render for Row {
    fn render(&self) -> Markup {
        html! {
            div .row style="display: flex; gap: 4px;" {
                @for cell in &self.cells {
                    (input_field(cell, self.is_disabled))
                }

                @if !self.is_disabled {
                    button .transparent .circle hx-put="/input" hx-include="input" hx-swap="innerHTML" hx-target="#grid-container"{
                        i { "arrow_forward" }
                    }
                }
            }
        }
    }
}

fn input_field(letter: &String, is_disabled: bool) -> Markup {
    html!(
        div .field .border .small .fill {
            input
                name="input[]"
                type="text"
                value=(letter)
                minlength="1"
                maxlength="1"
                required
                disabled[is_disabled]
                style="flex: 1; max-width: 60px; text-align: center;"
                oninput="this.value = this.value.toUpperCase().replace(/[^a-z]/gi, '');";
        }
    )
}
