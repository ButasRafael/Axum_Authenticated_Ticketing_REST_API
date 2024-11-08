use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::ctx::Ctx;

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForUpdate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    pub tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController{
    pub async fn new()->Result<Self>{
        Ok(Self{
            tickets_store:Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self,ctx:Ctx, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self,_ctx:Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        Ok(store.iter().filter_map(|t| t.clone()).collect())
    }

    pub async fn delete_ticket(&self, _ctx:Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
    pub async fn get_ticket(&self, _ctx:Ctx, id: u64) -> Result<Ticket> {
        let store = self.tickets_store.lock().unwrap();
        store.get(id as usize)
            .and_then(|t| t.clone())
            .ok_or(Error::TicketGetFailIdNotFound { id })
    }

    pub async fn update_ticket(&self, _ctx:Ctx, id: u64, ticket_update: TicketForUpdate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        if let Some(Some(ticket)) = store.get_mut(id as usize) {
            ticket.title = ticket_update.title;
            Ok(ticket.clone())
        } else {
            Err(Error::TicketUpdateFailIdNotFound { id })
        }
    }
}
