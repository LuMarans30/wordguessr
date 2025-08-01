use maud::{DOCTYPE, Markup, Render, html};

pub struct Layout {
    markup: Markup,
    title: String,
}

impl Layout {
    pub fn new(markup: Markup, title: String) -> Self {
        Self { markup, title }
    }

    fn head(&self) -> Markup {
        html! {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/beercss@latest/dist/cdn/beer.min.css";
            link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>💬</text></svg>";

            script type="module" src="https://cdn.jsdelivr.net/npm/beercss@latest/dist/cdn/beer.min.js" {}
            script type="module" src="https://cdn.jsdelivr.net/npm/material-dynamic-colors@latest/dist/cdn/material-dynamic-colors.min.js" {}
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}
            script src="https://cdn.jsdelivr.net/npm/htmx-ext-ws@2.0.2" {}


            title { (self.title) }
        }
    }

    fn appbar(&self) -> Markup {
        html! {
            header .primary {
                nav {
                    h4 .max .center-align { (self.title) }
                }
            }
        }
    }
}

impl Render for Layout {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    (self.head())
                }
                body hx-ext="ws" ws-connect="/ws" {
                    (self.appbar())
                    main .responsive .container {
                        (self.markup)
                    }
                }
            }
        }
    }
}
