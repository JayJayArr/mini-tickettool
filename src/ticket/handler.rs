use socketioxide::extract::{Data, SocketRef, State};
use tracing::info;

use crate::{Db, ticket::ticket_service::TicketRepository};

use super::model::TicketDraft;

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
) {
    info!(ns = socket.ns(), ?socket.id, "request new ticket");
    let _ = socket.emit(
        "tickets",
        &state
            .ticketrepo
            .lock()
            .await
            .create_ticket(ticketdraft)
            .await,
    );
}
