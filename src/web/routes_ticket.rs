use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate, TicketForUpdate};
use crate::Result;

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx:Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx,ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
    ctx:Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx:Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx,id).await?;
    Ok(Json(ticket))
}

async fn get_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - get_ticket", "HANDLER");
    let ticket = mc.get_ticket(ctx,id).await?;
    Ok(Json(ticket))
}

async fn update_ticket(
    State(mc): State<ModelController>,
    ctx:Ctx,
    Path(id): Path<u64>,
    Json(ticket_fc): Json<TicketForUpdate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - update_ticket", "HANDLER");
    let ticket = mc.update_ticket(ctx,id, ticket_fc).await?;
    Ok(Json(ticket))
}

pub fn routes(mc:ModelController)->Router{
    Router::new()
        .route("/ticket",post(create_ticket))
        .route("/ticket",get(list_tickets))
        .route("/ticket/:id",delete(delete_ticket))
        .route("/ticket/:id",get(get_ticket))
        .route("/ticket/:id",put(update_ticket))
        .with_state(mc)
}