use maud::{Markup, Render, html};

use crate::model::dialog::Dialog;

impl Render for Dialog {
    fn render(&self) -> Markup {
        html!(
            div hx-swap-oob="innerHTML:#dialog-container" {
                dialog class={@if self.is_active {"active"}} id=(self.get_id()) {
                    h5 { (self.title) }
                    div { (self.message) }
                    nav .right-align .no-space {
                        button data-ui={"#"(self.get_id())} .transparent .link { "Ok" }
                    }
                }
            }
        )
    }
}
