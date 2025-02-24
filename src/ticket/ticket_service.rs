use std::collections::HashMap;

use super::{
    model::{Ticket, TicketDraft, TicketId},
    status::TicketStatus,
};

pub trait TicketRepository {
    async fn get_tickets(&self) -> Vec<Ticket>;
    async fn get_ticket_by_id(&self, ticket_id: &TicketId) -> Option<&Ticket>;
    async fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> Option<&mut Ticket>;
    async fn create_ticket(&mut self, create_ticket: TicketDraft) -> TicketId;
    async fn delete_ticket(&mut self, ticket_id: &TicketId) -> Option<Ticket>;
}

pub struct InMemTicketRepository {
    tickets: HashMap<TicketId, Ticket>,
    counter: u32,
}

impl InMemTicketRepository {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 0,
        }
    }
}

impl TicketRepository for InMemTicketRepository {
    async fn get_tickets(&self) -> Vec<Ticket> {
        self.tickets.values().cloned().collect()
    }
    async fn create_ticket(&mut self, create_ticket: TicketDraft) -> TicketId {
        self.counter += 1;
        let id = TicketId(self.counter);
        let ticket = Ticket {
            ticket_id: id,
            title: create_ticket.title,
            description: create_ticket.description,
            status: TicketStatus::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }
    async fn get_ticket_by_id(&self, ticket_id: &TicketId) -> Option<&Ticket> {
        self.tickets.get(ticket_id)
    }
    async fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(ticket_id)
    }
    async fn delete_ticket(&mut self, ticket_id: &TicketId) -> Option<Ticket> {
        self.tickets.remove(ticket_id)
    }
}
