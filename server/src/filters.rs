use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use tera::{Result, Value, to_value, try_get_value};

pub fn temperature_class(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let f = try_get_value!("temperature_class", "value", f64, value);

    // Round and compute the color based on the thresholds
    let color = if f < 62.0 {
        "cold"
    } else if f < 68.0 {
        "cool"
    } else if f < 74.0 {
        "good"
    } else if f < 84.0 {
        "warm"
    } else {
        "hot"
    };

    Ok(to_value(color).unwrap())
}

pub fn probability(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let f = try_get_value!("probability", "value", f64, value);

    Ok(to_value(format!("{}", (f * 100.0).round())).unwrap())
}

pub fn precip_class(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let f = try_get_value!("precip_class", "value", f64, value);

    let class = if f > 0.0 { "wet" } else { "dry" };

    Ok(to_value(class).unwrap())
}

pub fn time_ago(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let f = try_get_value!("time_ago", "value", i64, value);

    let now = Utc::now();
    let diff = now.signed_duration_since(Utc.timestamp_opt(f, 0).unwrap());

    let minutes = diff.num_minutes();
    let hours = diff.num_hours();

    let rendered = if minutes < 1 {
        "less than 1 minute".to_string()
    } else if minutes == 1 {
        "1 minute".to_string()
    } else if minutes < 60 {
        format!("{} minutes", minutes)
    } else if hours == 1 {
        "1 hour".to_string()
    } else {
        format!("{} hours", hours)
    };

    Ok(to_value(rendered).unwrap())
}
