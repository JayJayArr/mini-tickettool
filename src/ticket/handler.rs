use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State},
};
use tracing::info;

use crate::{Db, ticket::ticket_service::TicketRepository};

use super::model::{TicketDraft, TicketIdRequest};

pub async fn get_tickets(socket: SocketRef, state: State<Db>) {
    info!(ns = socket.ns(), ?socket.id, "request tickets");
    let _ = socket.emit(
        "tickets",
        &state.ticketrepo.lock().await.get_tickets().await,
    );
}

pub async fn create_ticket(
    socket: SocketRef,
    state: State<Db>,
    Data(ticketdraft): Data<TicketDraft>,
    io: SocketIo,
) {
    info!(ns = socket.ns(), ?socket.id, "request new ticket");
    let _ = &state
        .ticketrepo
        .lock()
        .await
        .create_ticket(ticketdraft)
        .await;

    let _ = io
        .emit(
            "tickets",
            &state.ticketrepo.lock().await.get_tickets().await,
        )
        .await;
}

pub async fn delete_ticket(
    socket: SocketRef,
    state: State<Db>,
    Data(ticket_id): Data<TicketIdRequest>,
) {
    info!(ns = socket.ns(), ?socket.id, "delete ticket");
    let _ = socket.emit(
        "tickets",
        &state
            .ticketrepo
            .lock()
            .await
            .delete_ticket(&ticket_id.id)
            .await,
    );
}
