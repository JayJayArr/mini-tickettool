use std::sync::Arc;

use axum::routing::get;
use db::Db;
use rmpv::Value;
use socketioxide::{
    SocketIoBuilder,
    extract::{Data, SocketRef},
};
use ticket::{
    handler::{create_ticket, delete_ticket, get_ticket_by_id, get_tickets, update_ticket},
    ticket_service::InMemTicketRepository,
};
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
mod db;
mod ticket;
mod user;

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();
    socket.on("tickets", get_tickets);
    socket.on("create_ticket", create_ticket);
    socket.on("delete_ticket", delete_ticket);
    socket.on("get_ticket_by_id", get_ticket_by_id);
    socket.on("update_ticket", update_ticket);
    socket.on_disconnect(on_disconnect);
}

fn on_disconnect(socket: SocketRef) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO disconnected");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db {
        ticketrepo: Arc::new(Mutex::new(InMemTicketRepository::new())),
    };
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIoBuilder::new().with_state(db).build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
