/*
 * Pirate Weather API
 *
 * Pirate Weather provides an open, free, and documented source of government weather data.
 *
 * The version of the OpenAPI document: 2.6.0
 * Contact: mail@pirateweather.net
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Hourly : A block containing hour-by-hour forecasted conditions for the next 48 hours. If `extend=hourly` is used, the hourly block gives hour-by-hour forecasted conditions for the next 168 hours.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hourly {
    /// A summary of the hourly forecast.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An icon representing the hourly forecast.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::HourlyDataInner>>,
}

impl Hourly {
    /// A block containing hour-by-hour forecasted conditions for the next 48 hours. If `extend=hourly` is used, the hourly block gives hour-by-hour forecasted conditions for the next 168 hours.
    pub fn new() -> Hourly {
        Hourly {
            summary: None,
            icon: None,
            data: None,
        }
    }
}
