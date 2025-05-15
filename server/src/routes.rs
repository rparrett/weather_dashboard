use axum::{Json, extract::State, http::StatusCode, response::Html};
use tera::Context;
use tracing::{debug, error};

use crate::AppState;

pub(crate) async fn is_data_fresh(State(state): State<AppState>) -> Result<Json<bool>, StatusCode> {
    let forecast_cache = state.forecast_cache.read().await;
    Ok(Json(!forecast_cache.needs_update()))
}

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let needs_update = {
        let forecast_cache = state.forecast_cache.read().await;
        if forecast_cache.needs_update() {
            debug!("Waking background worker");
            state.notify.notify_one();
            true
        } else {
            false
        }
    };

    let forecast_cache = state.forecast_cache.read().await;

    let mut context = Context::new();
    context.insert("forecasts", &forecast_cache.forecasts);
    context.insert("needs_update", &needs_update);
    context.insert("updated_at", &forecast_cache.updated_at);

    let rendered = state
        .tera
        .render("main.tera", &context)
        .inspect_err(|e| error!("Failed to render template. {}", e))
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(rendered))
}
