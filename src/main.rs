use axum::{
    extract::{
        ws::{Message, WebSocket},
        DefaultBodyLimit, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use error::AppError;
use tower_http::cors::CorsLayer;

pub mod error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // CORS
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    // Router
    let app: Router<()> = Router::new()
        .route("/", get(websocket_handler))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)); //100MB

    // Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await?;
    tracing::info!("listening on ws://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

//Handler
pub async fn websocket_handler(
    web_socket: WebSocketUpgrade,
) -> Result<impl IntoResponse, AppError> {
    Ok(web_socket.on_upgrade(|socket| async move {
        if let Err(error) = handle_socket(socket).await {
            tracing::error!("WebSocket error: {:?}", error);
        }
    }))
}

async fn handle_socket(mut socket: WebSocket) -> Result<(), AppError> {
    while let Some(message) = socket.recv().await {
        // Receive a message from the client
        match message {
            Ok(message) => match message {
                Message::Text(text) => {
                    tracing::info!("Received text: {}", text);
                }
                Message::Binary(binary) => {
                    tracing::info!("Received binary: {:?}", binary);
                }
                Message::Ping(ping) => {
                    tracing::info!("Received ping: {:?}", ping);
                }
                Message::Pong(pong) => {
                    println!("Received pong: {:?}", pong);
                }
                Message::Close(close) => {
                    tracing::info!("Client disconnected: {:?}", close);
                    println!("Client disconnected");
                    return Ok(());
                }
            },
            Err(errer) => {
                tracing::error!("Error receiving message: {}", errer);
                return Err(AppError::from(errer));
            }
        }
        // Send a message to the client
        socket
            .send(Message::Text("Hello from server!".into()))
            .await?;
    }
    Ok(())
}
