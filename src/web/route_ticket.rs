use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use crate::ctx::Ctx;

// when ctx is last arguments it show error
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!(">> {:<12} CREATE ", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!(">> {:<12} LIST", "HANDLER");

    let ticket = mc.list_tickets(ctx).await?;

    Ok(Json(ticket))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
    ctx: Ctx
) -> Result<Json<Ticket>> {
    println!(">> {:<12} DELETE", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_ticket))
        .with_state(mc)
}
