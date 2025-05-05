use serde::{Deserialize, Serialize};

use super::status::TicketStatus;

#[derive(Clone, Serialize, Ord, PartialEq, Eq, PartialOrd)]
pub struct Ticket {
    pub ticket_id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: TicketStatus,
    // assignee: Option<User>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TicketUpdateDraft {
    pub ticket_id: TicketId,
    pub status: TicketStatus,
    pub title: TicketTitle,
    pub description: TicketDescription,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketTitle(pub String);
#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialEq, Eq, PartialOrd)]
pub struct TicketDescription(pub String);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Debug, Deserialize, PartialOrd, Ord)]
pub struct TicketId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Debug, Deserialize)]
pub struct TicketIdRequest {
    pub id: TicketId,
}
