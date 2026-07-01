mod ctx;
mod errors;
mod model;
mod web;
mod log;

// re export
pub use self::errors::{Error, Result};

use crate::model::ModelController;
use axum::response::Response;
use axum::{
    extract::{Path, Query}, http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::{get, get_service, post},
    Json,
    Router,
};
use axum::http::{Method, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{net, signal};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::log::log_request;

#[tokio::main]
async fn main() {
    let mc = ModelController::new().await.unwrap();

    // Auth Middleware only applicable to ticket api
    // So it must be seperated from main router
    let route_apis = web::route_ticket::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/users", post(create_user))
        .route("/params", get(params_test))
        .nest_service("/static_files", get_service(ServeDir::new("./")))
        .route("/path_test/{path}", get(path_params))
        .merge(web::route_login::routes())
        .nest("/api", route_apis)
        // Layers executed from bottom to top
        // cookie later data available in upper layers
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());

    let listener = net::TcpListener::bind("localhost:8080").await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            signal::ctrl_c()
                .await
                .expect("Could not register ctrl+c handler");
        })
        .await
        .unwrap();
}

// middleware
async fn main_response_mapper(ctx: Result<Ctx>, uri: Uri, req_method: Method, res: Response) -> Response {
    println!(">> {:<12} Response Mapper", "MIDDLEWARE");

    let uuid = Uuid::new_v4();

    let service_err = res.extensions().get::<Error>();
    let client_status_err = service_err.map(|se| se.client_status_and_error());

    let err_res = client_status_err
        .as_ref()
        .map(|(status_code, client_error)| {
            let err_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });
            println!(">> Client Error: {err_body}");

            // when deref,we take ownership by Copy trait
            (*status_code, Json(err_body)).into_response()
        });

    // Uzip to get Option of client Err from the tuple
    println!(">> SERVER LOG: {uuid} | Error: {service_err:?}");

    let client_err = client_status_err.unzip().1;
    log_request(uuid, req_method, uri, ctx.ok(), service_err, client_err).await;

    err_res.unwrap_or(res)
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

async fn create_user(Json(CreateUser { name }): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User { id: 1000, name };

    (StatusCode::CREATED, Json(user))
}

// =============== Tutorial ===============

#[derive(Deserialize, Debug)]
struct Params {
    name: Option<String>,
}

async fn params_test(Query(params): Query<Params>) -> impl IntoResponse {
    println!(">> {:<12} param_test - {params:?}", "HANDLER");

    // no new string alloc, but create new Option
    let name = params.name.as_deref().unwrap_or("Test User");

    Html(format!("<h1>Hello, {name}!</h1>"))
}

async fn path_params(Path(params): Path<String>) -> impl IntoResponse {
    let path = params.as_str();
    Html(format!("<h1>{path}</h1>"))
}

// Compose routes
#[allow(dead_code, unused_variables)]
async fn composer() {
    let rt_all = Router::new()
        .merge(route_static())
        .fallback_service(fallback());

    fn route_static() -> Router {
        Router::new().nest_service("/static/files", get_service(ServeDir::new("./")))
    }

    fn fallback() -> Router {
        Router::new().nest_service("/404", get(|| async { "Page not found!" }))
    }
}
