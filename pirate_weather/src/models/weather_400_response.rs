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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Weather400Response {
    /// The error message
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Weather400Response {
    pub fn new() -> Weather400Response {
        Weather400Response { detail: None }
    }
}
