use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TicketStatus {
    ToDo,
    Doing,
    Done,
}
