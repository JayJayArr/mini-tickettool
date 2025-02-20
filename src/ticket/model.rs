// use crate::user::model::User;

use super::status::TicketStatus;

#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
    // assignee: User
}

pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
#[derive(Clone)]
pub struct TicketTitle(String);
#[derive(Clone)]
pub struct TicketDescription(String);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TicketId(pub u32);
