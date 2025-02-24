// use crate::user::model::User;

use serde::{Deserialize, Serialize};

use super::status::TicketStatus;

#[derive(Clone, Serialize)]
pub struct Ticket {
    pub ticket_id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
    // assignee: User
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicketTitle(String);
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TicketDescription(String);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Debug)]
pub struct TicketId(pub u32);
