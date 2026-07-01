use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::{Error, Result};
use crate::ctx::Ctx;

// clone to send to client
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
// only clone arc
pub struct ModelController {
    // List items will replace by none
    // This grows infinity, not use in production
    // only for local Testing
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

// Constructor
impl ModelController {
    // since using Arc default we could have use drive on struct
    // but we can control the signature of the constructor
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default()
        })
    }
}

// CRUD
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();

        let tickets = store.iter()
            .filter_map(|x| x.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize)
            .and_then(|x| x.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

