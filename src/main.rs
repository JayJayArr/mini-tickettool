use std::sync::Arc;

use axum::routing::get;
use rmpv::Value;
use socketioxide::{
    SocketIo, SocketIoBuilder,
    extract::{Data, SocketRef, State},
};
use ticket::{
    handler::{create_ticket, get_tickets},
    ticket_service::InMemTicketRepository,
};
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
mod ticket;
mod user;
use user::user_service::InMemUserRepository;

#[derive(Clone)]
struct Db {
    ticketrepo: Arc<Mutex<InMemTicketRepository>>,
    userrepo: Arc<Mutex<InMemUserRepository>>,
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();
    // socket.on("tickets", async |socket: SocketRef, state: State<Db>| {
    //     info!(ns = socket.ns(), ?socket.id, "requets tickets");
    //     socket.emit(
    //         "tickets",
    //         &state.ticketrepo.lock().await.get_tickets().await,
    //     );
    // });
    socket.on("tickets", get_tickets);
    socket.on("create_ticket", create_ticket);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db {
        ticketrepo: Arc::new(Mutex::new(InMemTicketRepository::new())),
        userrepo: Arc::new(Mutex::new(InMemUserRepository::new())),
    };
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIoBuilder::new().with_state(db).build_layer();
    // let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
