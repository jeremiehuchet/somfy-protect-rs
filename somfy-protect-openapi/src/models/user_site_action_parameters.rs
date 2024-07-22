/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSiteActionParameters {
    /// Precision.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    /// Latitude of the checkInOut event.
    #[serde(rename = "event_latitude", skip_serializing_if = "Option::is_none")]
    pub event_latitude: Option<String>,
    /// Longitude of the checkInOut event.
    #[serde(rename = "event_longitude", skip_serializing_if = "Option::is_none")]
    pub event_longitude: Option<String>,
    /// Precision of the checkInOut event.
    #[serde(rename = "event_precision", skip_serializing_if = "Option::is_none")]
    pub event_precision: Option<String>,
    /// Latitude of the geofence.
    #[serde(rename = "geofence_latitude", skip_serializing_if = "Option::is_none")]
    pub geofence_latitude: Option<String>,
    /// Longitude of the geofence.
    #[serde(rename = "geofence_longitude", skip_serializing_if = "Option::is_none")]
    pub geofence_longitude: Option<String>,
    /// Radius of the geofence.
    #[serde(rename = "geofence_radius", skip_serializing_if = "Option::is_none")]
    pub geofence_radius: Option<String>,
    /// Distance.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<String>,
    /// Phone ID.
    #[serde(rename = "phone_id", skip_serializing_if = "Option::is_none")]
    pub phone_id: Option<String>,
}

impl UserSiteActionParameters {
    pub fn new() -> UserSiteActionParameters {
        UserSiteActionParameters {
            precision: None,
            event_latitude: None,
            event_longitude: None,
            event_precision: None,
            geofence_latitude: None,
            geofence_longitude: None,
            geofence_radius: None,
            distance: None,
            phone_id: None,
        }
    }
}
