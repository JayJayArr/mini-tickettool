// use crate::user::model::User;

use super::status::TicketStatus;

#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: TicketId,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    // assignee: User
}

pub struct CreateTicket {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TicketId(pub u32);
