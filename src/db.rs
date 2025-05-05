use std::sync::Arc;

use tokio::sync::Mutex;

use crate::ticket::ticket_service::InMemTicketRepository;

#[derive(Clone)]
pub struct Db {
    pub ticketrepo: Arc<Mutex<InMemTicketRepository>>,
    // userrepo: Arc<Mutex<InMemUserRepository>>,
}
