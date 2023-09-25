use axum::{ response::{ IntoResponse, Response }, http::StatusCode };

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
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

// region:    --- Froms
// impl From<model::Error> for Error {
// 	fn from(val: model::Error) -> Self {
// 		Self::Model(val)
// 	}
// }
// endregion: --- Froms

// region:    --- Error Boilerplate
// impl core::fmt::Display for Error {
// 	fn fmt(
// 		&self,
// 		fmt: &mut core::fmt::Formatter,
// 	) -> core::result::Result<(), core::fmt::Error> {
// 		write!(fmt, "{self:?}")
// 	}
// }

// impl std::error::Error for Error {}
// // endregion: --- Error Boilerplate
