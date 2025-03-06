use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State},
};
use tracing::info;

use crate::{Db, ticket::ticket_service::TicketRepository};

use super::model::{TicketDraft, TicketIdRequest, TicketUpdateDraft};

pub async fn get_tickets(socket: SocketRef, state: State<Db>) {
    info!(ns = socket.ns(), ?socket.id, "request tickets");
    let _ = socket.emit(
        "tickets",
        &state.ticketrepo.lock().await.get_tickets().await.sort(),
    );
}

pub async fn create_ticket(
    socket: SocketRef,
    state: State<Db>,
    Data(ticketdraft): Data<TicketDraft>,
    io: SocketIo,
) {
    info!(ns = socket.ns(), ?socket.id, "request new ticket");
    let ticketrepo = &mut state.ticketrepo.lock().await;
    let _ = socket.emit("tickets", &ticketrepo.create_ticket(ticketdraft).await);

    let _ = io
        .emit("tickets", &ticketrepo.get_tickets().await)
        .await
        .ok();
    // io.emit("tickets", &ticketrepo.get_tickets().await.sort())
    //     .await;
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

pub async fn get_ticket_by_id(
    socket: SocketRef,
    state: State<Db>,
    Data(ticket_id): Data<TicketIdRequest>,
) {
    info!(ns = socket.ns(), ?socket.id, "get ticket by id");
    let _ = socket.emit(
        "tickets",
        &state
            .ticketrepo
            .lock()
            .await
            .get_ticket_by_id(&ticket_id.id)
            .await,
    );
}

pub async fn update_ticket(
    socket: SocketRef,
    state: State<Db>,
    Data(updatedraft): Data<TicketUpdateDraft>,
) {
    info!(ns = socket.ns(), ?socket.id, "update ticket");
    match state
        .ticketrepo
        .lock()
        .await
        .getmut_ticket_by_id(&updatedraft.ticket_id)
        .await
    {
        Some(ticket) => {
            ticket.description = updatedraft.description;
            ticket.title = updatedraft.title;
            ticket.status = updatedraft.status;
        }
        None => socket.emit("tickets", "ticket doesn't exist").unwrap(),
    };
}
