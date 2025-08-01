use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{Html, IntoResponse},
    routing::get,
};
use clap::{Parser, command};
use maud::{Markup, Render, html};
use serde::Deserialize;
use tokio::sync::RwLock;
use uuid::Uuid;

use std::{collections::HashMap, net::SocketAddr};
use std::{ops::ControlFlow, sync::Arc};

use futures_util::stream::StreamExt;

use color_eyre::Result;

use crate::{
    controller::{game_controller::GameController, input_controller::InputController},
    model::game_state::GameState,
    service::dictionary::{DictionaryService, WordService},
    view::layout::Layout,
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

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<RwLock<HashMap<Uuid, GameState>>>,
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

    // Create controllers
    let game_controller = Arc::new(GameController::new(word_service));
    let input_controller = Arc::new(InputController::new(Arc::clone(&game_controller)));

    let sessions = Arc::new(RwLock::new(HashMap::<Uuid, GameState>::new()));

    //First session
    let initial_session_id = Uuid::nil();
    let initial_game_state = game_controller
        .create_new_game(args.num_tries, args.word_length)
        .await?;
    sessions
        .write()
        .await
        .insert(initial_session_id, initial_game_state);

    Ok(AppState {
        sessions,
        input_controller,
        game_controller,
    })
}

async fn initialize_server(app_state: AppState) -> Result<()> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Server running on http://{}", listener.local_addr()?);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn root_handler(State(state): State<AppState>) -> Html<String> {
    match render_root(&state).await {
        Ok(markup) => Html(markup.into_string()),
        Err(_) => Html(render_error_page("Failed to load game").into_string()),
    }
}

async fn render_root(state: &AppState) -> Result<Markup> {
    let game_state = {
        state
            .sessions
            .read()
            .await
            .get(&Uuid::nil())
            .expect("Can't find init session")
            .clone()
    };

    let layout = Layout::new(game_state.render(), "WordGuessr".into());
    Ok(layout.render())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.into()))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let (num_tries, word_length) = {
        let mut sessions = state.sessions.write().await;
        let game_state = sessions
            .get_mut(&Uuid::nil())
            .expect("Can't find init session");
        (game_state.num_tries, game_state.word_length)
    };

    let game_state = state
        .game_controller
        .create_new_game(num_tries, word_length)
        .await
        .expect("Can't create new game");

    let session_id = Uuid::new_v4();
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(session_id, game_state);
    }

    while let Some(Ok(msg)) = socket.next().await {
        match process_message(&state, msg, session_id).await {
            ControlFlow::Break(()) => break,
            ControlFlow::Continue(html) => {
                socket.send(Message::Text(html.0.into())).await.unwrap();
            }
        }
    }

    state.sessions.write().await.remove(&session_id);
}

async fn process_message(
    state: &Arc<AppState>,
    msg: Message,
    session_id: Uuid,
) -> ControlFlow<(), Html<String>> {
    let msg = match msg {
        Message::Text(t) => t,
        Message::Close(_) => return ControlFlow::Break(()),
        _ => return ControlFlow::Continue(Html(String::new())),
    };

    let markup = if is_reset_request(&msg) {
        handle_reset(state, session_id).await
    } else if let Ok(input) = serde_json::from_str::<RowElements>(&msg) {
        handle_input(state, session_id, input).await
    } else {
        render_error_page("Invalid message format")
    };

    ControlFlow::Continue(Html(markup.into_string()))
}

async fn handle_input(state: &AppState, session_id: Uuid, input: RowElements) -> Markup {
    let mut sessions = state.sessions.write().await;
    if let Some(game_state) = sessions.get_mut(&session_id) {
        return match state
            .game_controller
            .process_guess(game_state, input.input)
            .await
        {
            Ok(_) => game_state.render(),
            Err(_) => render_error_page("Failed to process guess"),
        };
    }

    render_error_page("Session not found")
}

async fn handle_reset(state: &AppState, session_id: Uuid) -> Markup {
    let mut sessions = state.sessions.write().await;
    if let Some(game_state) = sessions.get_mut(&session_id) {
        let new_game_state = state
            .game_controller
            .create_new_game(game_state.num_tries, game_state.word_length)
            .await
            .expect("Couldn't create a new game");

        *game_state = new_game_state;

        return game_state.render();
    }

    render_error_page("Session not found")
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
        "Error".into(),
    );
    layout.render()
}

fn is_reset_request(msg: &str) -> bool {
    serde_json::from_str::<HashMap<String, serde_json::Value>>(msg)
        .map(|map| map.contains_key("reset"))
        .unwrap_or(false)
}
