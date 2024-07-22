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
pub struct GatewayStatus {
    /// Last online status.
    #[serde(rename = "last_online_at", skip_serializing_if = "Option::is_none")]
    pub last_online_at: Option<String>,
    /// Last ofline status.
    #[serde(rename = "last_offline_at", skip_serializing_if = "Option::is_none")]
    pub last_offline_at: Option<String>,
    /// Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "device_lost", skip_serializing_if = "Option::is_none")]
    pub device_lost: Option<bool>,
    /// Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "last_status_at", skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
}

impl GatewayStatus {
    pub fn new() -> GatewayStatus {
        GatewayStatus {
            last_online_at: None,
            last_offline_at: None,
            device_lost: None,
            last_status_at: None,
        }
    }
}
