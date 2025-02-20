use std::collections::HashMap;

use super::{
    model::{Ticket, TicketDraft, TicketId},
    status::TicketStatus,
};

trait TicketService {
    async fn get_tickets(&self) -> Vec<Ticket>;
    async fn get_ticket_by_id(&self, ticket_id: &TicketId) -> Option<&Ticket>;
    async fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> Option<&mut Ticket>;
    async fn create_ticket(&mut self, create_ticket: TicketDraft) -> Ticket;
    async fn delete_ticket(&mut self, ticket_id: &TicketId) -> Option<Ticket>;
}

pub struct InMemTicketRepository {
    tickets: HashMap<TicketId, Ticket>,
    counter: u32,
}

impl InMemTicketRepository {
    fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 1,
        }
    }
}

impl TicketService for InMemTicketRepository {
    async fn get_tickets(&self) -> Vec<Ticket> {
        self.tickets.values().cloned().collect()
    }
    async fn create_ticket(&mut self, create_ticket: TicketDraft) -> Ticket {
        self.counter += 1;
        let ticket = Ticket {
            ticket_id: TicketId(self.counter),
            title: create_ticket.title,
            description: create_ticket.description,
            status: TicketStatus::ToDo,
        };
        self.tickets.insert(TicketId(self.counter), ticket).unwrap()
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
