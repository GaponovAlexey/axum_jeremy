use axum::routing::{ post, delete };
use axum::{ Json, Router };
use axum::extract::{ State, Path };
use crate::ctx::Ctx;
use crate::{ Result };
use crate::model::{ ModelController, Ticket, TicketForCreate };

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_tickets).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_tickets(
    ctx: Ctx,
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}
async fn list_tickets(ctx: Ctx, State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("List tickets");
    // -
    let tickets = mc.list_ticket(ctx).await?;
    Ok(Json(tickets))
}
async fn delete_ticket(
    ctx: Ctx,
    State(mc): State<ModelController>,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    println!("delete tickets");
    let ticket = mc.delete_ticket(id, ctx).await?;
    Ok(Json(ticket))
}
