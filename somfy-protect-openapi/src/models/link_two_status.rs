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
pub struct LinkTwoStatus {
    /// Power state.
    #[serde(rename = "power_state", skip_serializing_if = "Option::is_none")]
    pub power_state: Option<i32>,
    /// Internal battery level.
    #[serde(
        rename = "internal_battery_level",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_battery_level: Option<i32>,
    /// Wifi Level.
    #[serde(rename = "wifi_level", skip_serializing_if = "Option::is_none")]
    pub wifi_level: Option<i32>,
    /// Wifi Level Percent.
    #[serde(rename = "wifi_level_percent", skip_serializing_if = "Option::is_none")]
    pub wifi_level_percent: Option<i32>,
    /// Last online status.
    #[serde(rename = "last_online_at", skip_serializing_if = "Option::is_none")]
    pub last_online_at: Option<String>,
    /// Last ofline status.
    #[serde(rename = "last_offline_at", skip_serializing_if = "Option::is_none")]
    pub last_offline_at: Option<String>,
    #[serde(rename = "power_mode", skip_serializing_if = "Option::is_none")]
    pub power_mode: Option<serde_json::Value>,
    /// Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "device_lost", skip_serializing_if = "Option::is_none")]
    pub device_lost: Option<bool>,
    /// Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "last_status_at", skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
}

impl LinkTwoStatus {
    pub fn new() -> LinkTwoStatus {
        LinkTwoStatus {
            power_state: None,
            internal_battery_level: None,
            wifi_level: None,
            wifi_level_percent: None,
            last_online_at: None,
            last_offline_at: None,
            power_mode: None,
            device_lost: None,
            last_status_at: None,
        }
    }
}
