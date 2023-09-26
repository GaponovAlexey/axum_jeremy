use axum::{ response::{ IntoResponse, Response }, http::StatusCode };

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LoginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    // -- Model
    TicketDeleteFailIdNotFound {
        id: u64,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("My Err ----- ");
        (StatusCode::INTERNAL_SERVER_ERROR, "Err").into_response()
    }
}

