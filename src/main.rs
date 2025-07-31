use axum::{
    Router,
    extract::{
        ConnectInfo, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{Html, IntoResponse},
    routing::{any, delete, get, put},
};
use axum_extra::extract::Form;
use clap::{Parser, command};
use maud::{Markup, Render, html};
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use std::{collections::HashMap, net::SocketAddr};
use std::{ops::ControlFlow, sync::Arc};

//allows to split the websocket stream into separate TX and RX branches
use futures_util::stream::StreamExt;

use color_eyre::Result;

use crate::{
    controller::{
        game_controller::{self, GameController},
        input_controller::InputController,
    },
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

#[derive(Deserialize, Debug)]
struct RowElements {
    #[serde(rename = "input[]")]
    input: Vec<char>,
}

#[derive(Deserialize, Debug)]
struct ResetMsg {
    reset: String,
}

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<Uuid, GameState>>>,
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

    let sessions = Arc::new(RwLock::new(HashMap::new()));

    Ok(AppState {
        sessions,
        game_state,
        input_controller,
        game_controller,
    })
}

async fn initialize_server(app_state: AppState) -> Result<()> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/ws", any(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("🚀 Server running on http://{}", listener.local_addr()?);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state.into()))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, mut state: Arc<AppState>) {
    // Main message loop
    while let Some(Ok(msg)) = socket.next().await {
        if process_message(&mut state, msg, who).await.is_break() {
            return;
        }
    }
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_message(
    state: &mut Arc<AppState>,
    msg: Message,
    who: SocketAddr,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
            if let Ok(input) = serde_json::from_str::<RowElements>(&t) {
                let mut game_state = state.game_state.write().await;
                dbg!(&input);
                /*                 dbg!(
                    state
                        .input_controller
                        .handle_input(&state.game_state, input.input)
                        .await
                        .unwrap()
                ); */

                let result = state
                    .game_controller
                    .process_guess(&mut game_state, input.input)
                    .await
                    .unwrap();

                dbg!(result);
                dbg!(&game_state.grid);
            } else if serde_json::from_str::<ResetMsg>(&t).is_ok() {
                let result = reset_game(state).await.unwrap();
                dbg!(result.into_string());
            }
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {who} sent close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    }
    ControlFlow::Continue(())
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

/* async fn reset_handler(State(mut state): State<AppState>) -> Html<String> {
    match reset_game(&mut state).await {
        Ok(markup) => Html(markup.into_string()),
        Err(_) => Html(render_game_error().into_string()),
    }
} */

async fn reset_game(state: &mut Arc<AppState>) -> Result<Markup> {
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
