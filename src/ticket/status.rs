use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub enum TicketStatus {
    ToDo,
    Doing,
    Done,
}
