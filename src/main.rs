use std::{sync::Arc, time::Duration};

use axum::{
    BoxError, Json,
    error_handling::HandleErrorLayer,
    response::{IntoResponse, Response},
    routing::get,
};
use db::Db;
use http::StatusCode;
use rmpv::Value;
use socketioxide::{
    SocketIoBuilder,
    extract::{Data, SocketRef},
};
use ticket::{
    handler::{
        counter, create_ticket, delete_ticket, get_ticket_by_id, get_tickets, update_ticket,
    },
    model::{Ticket, TicketDescription, TicketId, TicketTitle},
    status::TicketStatus,
    ticket_service::InMemTicketRepository,
};
use tokio::sync::Mutex;
use tower::{
    ServiceBuilder,
    buffer::BufferLayer,
    limit::RateLimitLayer,
    load_shed::{LoadShedLayer, error::Overloaded},
};
use tower_http::compression::CompressionLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::user::user_service::InMemUserRepository;
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
    socket.on("counter", counter);
    socket.on_disconnect(on_disconnect);
}

fn on_disconnect(socket: SocketRef) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO disconnected");
}
async fn hello_handler() -> impl IntoResponse {
    Json(Ticket {
        ticket_id: TicketId(42),
        status: TicketStatus::ToDo,
        title: TicketTitle("yeet".to_string()),
        description: TicketDescription("akdkflkjf45fkdjoiujoujweoio".to_string()),
    })
}
async fn handle_error(error: BoxError) -> Response {
    if error.is::<Overloaded>() {
        (StatusCode::TOO_MANY_REQUESTS, "calm down plzz").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Db {
        ticketrepo: Arc::new(Mutex::new(InMemTicketRepository::new())),
        userrepo: Arc::new(Mutex::new(InMemUserRepository::new())),
    };
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (socketlayer, io) = SocketIoBuilder::new().with_state(db).build_layer();

    io.ns("/", on_connect);

    let httprouter = axum::Router::new().route("/hello", get(hello_handler));

    let app = axum::Router::new().merge(httprouter).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_error))
            .layer(BufferLayer::new(2))
            .layer(LoadShedLayer::new())
            .layer(CompressionLayer::new())
            .layer(RateLimitLayer::new(60, Duration::from_secs(60)))
            .layer(socketlayer),
    );

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
