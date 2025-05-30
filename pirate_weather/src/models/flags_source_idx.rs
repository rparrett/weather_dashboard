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

/// FlagsSourceIdx : The X, Y coordinate and the lat/long coordinate for each model used to generate the forecast. Only returned when version>2.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlagsSourceIdx {
    #[serde(rename = "hrrr", skip_serializing_if = "Option::is_none")]
    pub hrrr: Option<Box<models::FlagsSourceIdxHrrr>>,
    #[serde(rename = "nbm", skip_serializing_if = "Option::is_none")]
    pub nbm: Option<Box<models::FlagsSourceIdxNbm>>,
    #[serde(rename = "gfs", skip_serializing_if = "Option::is_none")]
    pub gfs: Option<Box<models::FlagsSourceIdxGfs>>,
    #[serde(rename = "etopo", skip_serializing_if = "Option::is_none")]
    pub etopo: Option<Box<models::FlagsSourceIdxEtopo>>,
}

impl FlagsSourceIdx {
    /// The X, Y coordinate and the lat/long coordinate for each model used to generate the forecast. Only returned when version>2.
    pub fn new() -> FlagsSourceIdx {
        FlagsSourceIdx {
            hrrr: None,
            nbm: None,
            gfs: None,
            etopo: None,
        }
    }
}
