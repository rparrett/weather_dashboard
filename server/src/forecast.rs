use anyhow::Context;
use chrono::{Datelike, TimeZone, Weekday};
use chrono_tz::Tz;
use pirate_weather::apis::weather_api::{WeatherApi, WeatherApiClient};
use serde::Serialize;

use crate::config::Location;

#[derive(Serialize)]
pub(crate) struct TemplateForecast {
    pub(crate) days: Vec<TemplateDay>,
    pub(crate) location: Location,
}

#[derive(Serialize)]
pub(crate) struct TemplateDay {
    pub(crate) date: String,
    pub(crate) week_day: String,
    pub(crate) is_weekend: bool,
    pub(crate) icon: String,
    pub(crate) summary: String,
    pub(crate) apparent_temperature_low: f64,
    pub(crate) apparent_temperature_high: f64,
    pub(crate) temperature_low: f64,
    pub(crate) temperature_high: f64,
    pub(crate) precip_probability: f64,
    pub(crate) wind_speed: f64,
}

pub(crate) async fn get_forecast(
    client: &WeatherApiClient,
    key: &str,
    location: &Location,
) -> anyhow::Result<TemplateForecast> {
    let weather = client
        .weather(
            key,
            &format!("{},{}", location.latitude, location.longitude),
            Some("currently,minutely,alerts,hourly"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;

    let mut days = vec![];

    let tz: Tz = weather.timezone.unwrap().parse().unwrap();

    let daily = weather
        .daily
        .with_context(|| "No daily object in response")?;

    let data = daily.data.with_context(|| "No daily data in response")?;

    for day in data {
        let time = day
            .time
            .map(|timestamp| tz.timestamp_opt(timestamp as i64, 0))
            .with_context(|| "No day time")?;

        let time = time
            .single()
            .with_context(|| "Time zone conversion failed")?;

        days.push(TemplateDay {
            date: time.format("%b %d").to_string(),
            week_day: time.format("%a").to_string(),
            is_weekend: time.weekday() == Weekday::Sat || time.weekday() == Weekday::Sun,
            icon: day.icon.unwrap(),
            summary: day.summary.unwrap(),
            apparent_temperature_low: day.apparent_temperature_low.unwrap(),
            apparent_temperature_high: day.apparent_temperature_high.unwrap(),
            temperature_low: day.temperature_low.unwrap(),
            temperature_high: day.temperature_high.unwrap(),
            precip_probability: day.precip_probability.unwrap(),
            wind_speed: day.wind_speed.unwrap(),
        })
    }

    Ok(TemplateForecast {
        days,
        location: location.clone(),
    })
}
