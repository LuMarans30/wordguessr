use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    extract::State,
    response::Html,
    routing::{delete, get, put},
};

use axum_extra::extract::Form;
use clap::Parser;
use maud::{Markup, Render};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::RwLock};

use color_eyre::Result;

use crate::{
    controller::{game_controller::GameController, input_controller::InputController},
    model::game_state::GameState,
    service::dictionary::{DictionaryService, WordService},
    view::{home::root, layout::Layout},
};

mod controller;
mod model;
mod service;
mod view;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Word length
    #[clap(short, long, default_value_t = 6)]
    word_length: usize,
    /// Number of tries
    #[clap(short, long, default_value_t = 6)]
    num_tries: usize,
}

#[derive(Deserialize)]
struct RowElements {
    #[serde(rename = "input[]")]
    input: Vec<char>,
}

#[derive(Clone)]
pub struct AppState {
    pub game_state: Arc<RwLock<GameState>>,
    pub input_controller: Arc<InputController>,
    pub game_controller: Arc<GameController>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let app_state = create_app_state(Args::parse()).await?;
    initialize_server(app_state).await
}

async fn create_app_state(args: Args) -> Result<AppState> {
    // Create word service
    let word_service: Arc<dyn WordService> = Arc::new(DictionaryService::new().await?);

    // Create initial game state
    let secret_word = word_service.get_random_word(args.word_length).await?;

    let game_state = Arc::new(RwLock::new(GameState::new(
        secret_word,
        args.num_tries,
        args.word_length,
    )));

    // Create controllers
    let game_controller = Arc::new(GameController::new(word_service));
    let input_controller = Arc::new(InputController::new(Arc::clone(&game_controller)));

    Ok(AppState {
        game_state,
        input_controller,
        game_controller,
    })
}

async fn initialize_server(app_state: AppState) -> Result<()> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/input", put(input_handler))
        .route("/reset", delete(reset_handler))
        .with_state(app_state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running on http://{addr}");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler(State(state): State<AppState>) -> Html<String> {
    match render_root(&state).await {
        Ok(markup) => Html(markup.into_string()),
        Err(_) => Html(render_error_page("Failed to load game").into_string()),
    }
}

async fn render_root(state: &AppState) -> Result<Markup> {
    let game_state = state.game_state.read().await;
    let layout = Layout::new(
        root(&game_state.grid).await,
        "WordGuessr".into(),
        "💬 WordGuessr".into(),
    );
    Ok(layout.render())
}

async fn input_handler(
    State(state): State<AppState>,
    Form(params): Form<RowElements>,
) -> Html<String> {
    match state
        .input_controller
        .handle_input(&state.game_state, params.input)
        .await
    {
        Ok(markup) => Html(markup.into_string()),
        Err(_) => Html(render_game_error().into_string()),
    }
}

async fn reset_handler(State(mut state): State<AppState>) -> Html<String> {
    match reset_game(&mut state).await {
        Ok(markup) => Html(markup.into_string()),
        Err(_) => Html(render_game_error().into_string()),
    }
}

async fn reset_game(state: &mut AppState) -> Result<Markup> {
    let (word_length, num_tries) = {
        let game_state = state.game_state.read().await;
        (game_state.word_length, game_state.num_tries)
    };

    let new_game_state = state
        .game_controller
        .create_new_game(num_tries, word_length)
        .await?;

    {
        let mut game_state = state.game_state.write().await;
        *game_state = new_game_state;
    }

    let game_state = state.game_state.read().await;
    Ok(game_state.grid.render())
}

fn render_error_page(message: &str) -> Markup {
    use maud::html;

    let layout = Layout::new(
        html! {
            div .container .center-align {
                div .error .padding {
                    h4 { "⚠️ Error" }
                    p { (message) }
                    button onclick="window.location.reload()" .primary {
                        "Reload Page"
                    }
                }
            }
        },
        "Error - WordGuessr".into(),
        "💬 WordGuessr - Error".into(),
    );
    layout.render()
}

fn render_game_error() -> Markup {
    use maud::html;

    html! {
        div .container .center-align {
            div .error .padding {
                p { "Something went wrong. Please try again." }
                button hx-get="/" hx-swap="outerHTML" hx-target="body" .primary {
                    "Refresh Game"
                }
            }
        }
    }
}
