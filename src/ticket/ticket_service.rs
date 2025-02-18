use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, RwLock};

use super::{
    model::{CreateTicket, Ticket, TicketId},
    status::{self, TicketStatus},
};

trait TicketService {
    async fn get_tickets(&self) -> Vec<Ticket>;
    async fn get_ticket_by_id(&self, ticket_id: &TicketId) -> &Ticket;
    async fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> &mut Ticket;
    async fn create_ticket(&mut self, create_ticket: CreateTicket) -> Ticket;
    async fn delete_ticket(&self, ticket_id: &TicketId) -> Ticket;
}

pub struct InMemTicketService {
    hashmap: Arc<Mutex<HashMap<TicketId, Ticket>>>,
    counter: u32,
}

impl InMemTicketService {
    fn new() -> Self {
        Self {
            hashmap: Arc::new(Mutex::new(HashMap::new())),
            counter: 0,
        }
    }
}

impl TicketService for InMemTicketService {
    async fn get_tickets(&self) -> Vec<Ticket> {
        self.hashmap.lock().await.values().cloned().collect()
    }
    async fn create_ticket(&mut self, create_ticket: CreateTicket) -> Ticket {
        self.counter = self.counter + 1;
        let ticket = Ticket {
            ticket_id: TicketId(self.counter),
            title: create_ticket.title,
            description: create_ticket.description,
            status: TicketStatus::ToDo,
        };
        self.hashmap
            .lock()
            .await
            .insert(TicketId(self.counter), ticket)
            .unwrap()
    }
    async fn get_ticket_by_id(&self, ticket_id: &TicketId) -> &Ticket {
        // self.hashmap.read().await.get(ticket_id)
        todo!()
    }
    async fn getmut_ticket_by_id(&mut self, ticket_id: &TicketId) -> &mut Ticket {
        // self.hashmap.lock().await.get_mut(ticket_id).unwrap()
        todo!()
    }
    async fn delete_ticket(&self, ticket_id: &TicketId) -> Ticket {
        self.hashmap.lock().await.remove(ticket_id).unwrap()
    }
}
