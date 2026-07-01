use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use crate::ctx::Ctx;
use crate::model::ModelController;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::{body::Body, http::Request, middleware::Next, response::Response, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

// when Option<ctx> first arg show error
pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!(">> {:<12} AUTH - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!(">> {:<12} RESOLVER", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let res_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };

    // Remove cookie if something went wring
    if res_ctx.is_err() && !matches!(res_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store res_ctx in request extension
    req.extensions_mut().insert(res_ctx);

    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        println!(">> {:<12} CTX", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r"^user-(\d+)\.(.+)\.(.+)", &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id = user_id
        .parse::<u64>()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
