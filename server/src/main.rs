use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use axum::{Router, routing::get};
use chrono::Utc;
use config::Config;
use forecast::{TemplateForecast, get_forecast};
use tera::Tera;
use tokio::{
    sync::{Notify, RwLock},
    task,
};
use worker::background_worker;

mod config;
mod filter;
mod forecast;
mod routes;
mod worker;

const MIN_UPDATE_FREQUENCY: i64 = 300;

#[derive(Default)]
struct ForecastCache {
    forecasts: Vec<TemplateForecast>,
    updated_at: i64,
}
impl ForecastCache {
    fn update(&mut self, new_forecasts: Vec<TemplateForecast>) {
        self.forecasts.clear();
        self.forecasts.extend(new_forecasts);
        self.updated_at = Utc::now().timestamp()
    }
    fn needs_update(&self) -> bool {
        Utc::now().timestamp() - self.updated_at > MIN_UPDATE_FREQUENCY
    }
}

#[derive(Clone, Default)]
struct AppState {
    tera: Tera,
    forecast_cache: Arc<RwLock<ForecastCache>>,
    config: Config,
    notify: Arc<Notify>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_dir = Path::new("server");
    let path = if server_dir.is_dir() {
        PathBuf::from("server")
    } else {
        PathBuf::new()
    };

    let config = Config::new(path.join("config.toml")).context("Failed to load config.toml")?;

    let mut tera = Tera::new(
        path.join("templates/**/*.tera")
            .to_str()
            .context("Failed to create template glob")?,
    )
    .context("Failed to load templates")?;

    tera.register_filter("temperature_class", filter::temperature_class);
    tera.register_filter("probability", filter::probability);
    tera.register_filter("precip_class", filter::precip_class);
    tera.register_filter("time_ago", filter::time_ago);

    let state = AppState {
        tera,
        config: config.clone(),
        ..Default::default()
    };

    task::spawn(background_worker(state.clone()));

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/is_data_fresh", get(routes::is_data_fresh))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.listen_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
