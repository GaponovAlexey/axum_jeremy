use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::{ Error, Result };
use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>
) -> Result<Response> {
    println!("cookie");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // -- parse token
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    Ok(next.run(req).await)
}

// -- region ---ctx extractor async trait
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// -- end-region: ---ctx extractor

// -- Parse token of format `user-[user-id].[expiration].[signature]
// -- Return (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(
        Error::AuthFailTokenWrongFormat
    )?;
    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
