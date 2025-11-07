use std::{fmt, sync::Arc};

use axum::{Router, extract::State, http::StatusCode, routing::post};
use tower_service::Service;
use worker::*;

struct AppState {
    pub db: D1Database,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState").finish_non_exhaustive()
    }
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/backups", post(post_backup))
        .with_state(Arc::new(state))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let state = AppState { db: env.d1("DB")? };
    Ok(router(state).call(req).await?)
}

#[axum::debug_handler]
pub async fn post_backup(State(state): State<Arc<AppState>>) -> StatusCode {
    todo!()
}
