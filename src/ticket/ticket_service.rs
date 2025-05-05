use std::collections::HashMap;

use super::{
    model::{Ticket, TicketDraft, TicketId},
    status::TicketStatus,
};

pub trait TicketRepository {
    fn get_tickets(&self) -> Vec<Ticket>;
    fn get_ticket_by_id(&self, ticket_id: &TicketId) -> Option<&Ticket>;
    fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> Option<&mut Ticket>;
    fn create_ticket(&mut self, create_ticket: TicketDraft) -> TicketId;
    fn delete_ticket(&mut self, ticket_id: &TicketId) -> Option<Ticket>;
    fn counter(&mut self) -> u32;
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
    fn get_tickets(&self) -> Vec<Ticket> {
        self.tickets.values().cloned().collect()
    }
    fn create_ticket(&mut self, create_ticket: TicketDraft) -> TicketId {
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
    fn get_ticket_by_id(&self, ticket_id: &TicketId) -> Option<&Ticket> {
        self.tickets.get(ticket_id)
    }
    fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(ticket_id)
    }
    fn delete_ticket(&mut self, ticket_id: &TicketId) -> Option<Ticket> {
        self.tickets.remove(ticket_id)
    }

    fn counter(&mut self) -> u32 {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use crate::ticket::{
        model::{TicketDescription, TicketDraft, TicketId, TicketTitle},
        ticket_service::TicketRepository,
    };

    use super::InMemTicketRepository;

    #[test]
    fn create_ticket() {
        let ticketdraft = TicketDraft {
            title: TicketTitle("first ticket".to_string()),
            description: TicketDescription("first description".to_string()),
        };
        let mut repo = InMemTicketRepository::new();
        assert_eq!(repo.counter(), 0);
        let id = repo.create_ticket(ticketdraft.clone());
        assert_eq!(repo.counter(), 1);
        assert_eq!(repo.get_ticket_by_id(&id).unwrap().title, ticketdraft.title);
        assert_eq!(
            repo.get_ticket_by_id(&id).unwrap().description,
            ticketdraft.description
        );
    }
    #[test]
    fn non_existing_ticket() {
        let repo = InMemTicketRepository::new();
        let result = repo.get_ticket_by_id(&TicketId(1));
        assert!(result.is_none())
    }

    #[test]
    fn delete_ticket() {
        let mut repo = InMemTicketRepository::new();
        let ticketdraft = TicketDraft {
            title: TicketTitle("first ticket".to_string()),
            description: TicketDescription("first description".to_string()),
        };
        assert_eq!(repo.counter(), 0);
        let id = repo.create_ticket(ticketdraft);
        assert_eq!(repo.counter(), 1);
        assert!(repo.get_ticket_by_id(&id).is_some());
        let result = repo.delete_ticket(&id);
        assert!(result.is_some());
        assert!(repo.get_ticket_by_id(&id).is_none());
    }
}
