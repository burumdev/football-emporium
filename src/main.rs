use std::sync::Arc;

use anyhow::Context;
use axum::Router;

use tower_http::services::ServeDir;

mod env;
use env::AppEnvVars;

mod imdb;
mod rest_api;

mod appstate;
use appstate::AppState;

pub mod constants;
use constants::ERR_PFX;

const MOD: &str = "MAIN";

async fn init_app() -> anyhow::Result<(Router, Arc<AppEnvVars>)> {
    const ERR_FN: &str = "::init_app";
    let app_state = AppState::init()
        .await
        .with_context(|| format!("{ERR_PFX} {MOD}{ERR_FN}: Initialization of app state failed."))?;

    let serve_dir = ServeDir::new("ui/dist");
    let env_vars = app_state.env_vars.clone();
    let rest_routes = rest_api::get_router(app_state);

    Ok((
        Router::new()
            .fallback_service(serve_dir)
            .nest("/api", rest_routes),
        env_vars,
    ))
}

#[tokio::main]
async fn main() {
    let (app, env_vars) = init_app().await.unwrap();

    let host_address: &str = env_vars.host_address.as_ref();
    let listener = tokio::net::TcpListener::bind(host_address).await.unwrap();

    println!("{MOD}: Server listening to address: {host_address}");

    axum::serve(listener, app).await.unwrap();
}
