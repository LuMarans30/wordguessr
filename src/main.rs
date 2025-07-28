use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    extract::State,
    routing::{delete, get, put},
};

use axum_extra::extract::Form;
use maud::{Markup, Render};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::Mutex};

use color_eyre::Result;

use crate::{
    model::grid::Grid,
    view::{home::root, layout::Layout},
};

mod view {
    pub mod home;
    pub mod layout;
    pub mod components {
        pub mod grid;
        pub mod row;
    }
}

mod model {
    pub mod grid;
    pub mod row;
}

#[derive(Clone)]
pub struct AppState {
    pub grid: Arc<Mutex<Grid>>,
}

#[derive(Deserialize)]
struct RowElements {
    #[serde(rename = "input[]")]
    input: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let app_state = AppState {
        grid: Arc::new(Mutex::new(Grid::default())),
    };

    initialize_server(app_state).await
}

async fn initialize_server(app_state: AppState) -> Result<()> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/input", put(input_handler))
        .route("/reset", delete(reset_handler))
        .with_state(app_state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler(State(state): State<AppState>) -> Markup {
    let grid = state.grid.lock().await;
    let layout = Layout::new(
        root(&grid).await,
        "WordGuessr".into(),
        "💬 wordguessr".into(),
    );
    layout.render()
}

async fn input_handler(State(state): State<AppState>, Form(params): Form<RowElements>) -> Markup {
    let mut grid = state.grid.lock().await;
    let current_row_index = grid.current_row;
    if !params.input.iter().any(String::is_empty) {
        grid.rows[current_row_index].cells = params.input;
        //TODO: match word with secret word
        grid.set_next_row();
    }
    grid.render()
}

async fn reset_handler(State(state): State<AppState>) -> Markup {
    let mut grid = state.grid.lock().await;
    grid.reset();
    grid.render()
}
