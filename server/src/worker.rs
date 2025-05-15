use pirate_weather::apis::{configuration::Configuration, weather_api::WeatherApiClient};

use crate::{AppState, get_forecast};

pub(crate) async fn background_worker(state: AppState) {
    loop {
        // Wait for notification
        state.notify.notified().await;

        println!("Background worker awake.");

        {
            let forecast_cache = state.forecast_cache.read().await;
            if !forecast_cache.needs_update() {
                println!("Nothing to do...");
                continue;
            }
        }

        println!("Fetching new data...");

        let weather_config = Configuration::default();
        let client = WeatherApiClient::new(weather_config.into());

        let mut ok = true;

        let mut forecasts = vec![];
        for location in &state.config.locations {
            let forecast =
                match get_forecast(&client, &state.config.pirate_weather_key, location).await {
                    Ok(forecast) => forecast,
                    Err(e) => {
                        println!("{:?}", e);
                        ok = false;
                        break;
                    }
                };
            forecasts.push(forecast);
        }

        if !ok {
            println!("Failed at least once, will retry later...");
            continue;
        }

        {
            let mut forecast_cache = state.forecast_cache.write().await;
            forecast_cache.update(forecasts);
        }

        println!("Update complete.");
    }
}
