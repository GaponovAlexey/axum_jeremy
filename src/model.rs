use crate::{ Error, Result };

use serde::{ Deserialize, Serialize };
use std::sync::{ Arc, Mutex };

//region ---Ticket Types---
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}
#[derive(Clone, Debug, Serialize)]
pub struct TicketForCreate {
    pub title: String,
}
//region --- Model Controller ---
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self { tickets_store: Arc::default() })
    }
}

// CRUD IMPL
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket { id, title: ticket_fc.title };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }
    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store
            .iter()
            .filter_map(|t| t.clone())
            .collect();
        Ok(tickets)
    }
    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let tickets = store.get_mut(id as usize).and_then(|t| t.take());
        tickets.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

//endregion ---da---
