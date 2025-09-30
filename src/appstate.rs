use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    env::AppEnvVars,
    imdb::{self, IMDB},
};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub imdb: Arc<IMDB<imdb::ReadyState>>,
    pub env_vars: Arc<AppEnvVars>,
}

impl AppState {
    pub async fn init() -> anyhow::Result<Self> {
        let env_vars = Arc::new(AppEnvVars::init()?);
        let imdb = Arc::new(IMDB::init().await?);

        Ok(Self { env_vars, imdb })
    }
}
