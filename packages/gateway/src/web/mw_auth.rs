
use crate::web::AUTH_TOKEN;
use crate::{web, Error, Result};
use crate::ctx::Ctx;
use tower_cookies::{Cookie, Cookies};
use axum::response::Response;
use axum::middleware::Next;
use axum::extract::Request;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use async_trait::async_trait;
use lazy_regex::regex_captures;


pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    ctx?;
    
    Ok(next.run(req).await)
}


#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailedNoAuthTokenCookie)
            .and_then(parse_token)?;
            
        Ok(Ctx::new(user_id))
    }

}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
		&token
	)
    .ok_or(Error::AuthFailedTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailedTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}