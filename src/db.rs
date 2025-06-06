use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    ticket::ticket_service::InMemTicketRepository, user::user_service::InMemUserRepository,
};

#[derive(Clone)]
pub struct Db {
    pub ticketrepo: Arc<Mutex<InMemTicketRepository>>,
    pub userrepo: Arc<Mutex<InMemUserRepository>>,
}
