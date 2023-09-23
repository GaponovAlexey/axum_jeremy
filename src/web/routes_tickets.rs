use axum::Json;
use axum::extract::State;
use crate::{ Error, Result };
use crate::model::{ ModelController, Ticket, TicketForCreate };

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("Create_ticket");
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("Create_ticket");
    let ticket = mc.create_ticket(ticket_fc).await?;
    todo!()
}
