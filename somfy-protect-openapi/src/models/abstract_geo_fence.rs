/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AbstractGeoFence {
    /// Latitude.
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// Longitude.
    #[serde(rename = "longitude")]
    pub longitude: f32,
    /// Radius.
    #[serde(rename = "radius")]
    pub radius: f32,
    /// Trigger exit.
    #[serde(rename = "trigger_exit", skip_serializing_if = "Option::is_none")]
    pub trigger_exit: Option<bool>,
    /// Trigger exit if last.
    #[serde(
        rename = "trigger_exit_if_last",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_exit_if_last: Option<bool>,
    /// Trigger enter.
    #[serde(rename = "trigger_enter", skip_serializing_if = "Option::is_none")]
    pub trigger_enter: Option<bool>,
    /// Mobile ID used for smart activation and presence. (nullable).
    #[serde(rename = "location_phone_id", skip_serializing_if = "Option::is_none")]
    pub location_phone_id: Option<String>,
}

impl AbstractGeoFence {
    pub fn new(latitude: f32, longitude: f32, radius: f32) -> AbstractGeoFence {
        AbstractGeoFence {
            latitude,
            longitude,
            radius,
            trigger_exit: None,
            trigger_exit_if_last: None,
            trigger_enter: None,
            location_phone_id: None,
        }
    }
}
