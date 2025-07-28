use maud::{DOCTYPE, Markup, Render, html};

pub struct Layout {
    markup: Markup,
    appbar_title: String,
    webpage_title: String,
}

impl Layout {
    pub fn new(markup: Markup, appbar_title: String, webpage_title: String) -> Self {
        Self {
            markup,
            webpage_title,
            appbar_title,
        }
    }

    fn head(&self) -> Markup {
        html! {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/beercss@latest/dist/cdn/beer.min.css";
            script type="module" src="https://cdn.jsdelivr.net/npm/beercss@latest/dist/cdn/beer.min.js" {}
            script type="module" src="https://cdn.jsdelivr.net/npm/material-dynamic-colors@latest/dist/cdn/material-dynamic-colors.min.js" {}
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js" {}

            title { (self.webpage_title) }
        }
    }

    fn appbar(&self) -> Markup {
        html! {
            header .primary {
                nav {
                    h4 .max .center-align { (self.appbar_title) }
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
                body {
                    (self.appbar())
                    main .responsive {
                        (self.markup)
                    }
                }
            }
        }
    }
}
