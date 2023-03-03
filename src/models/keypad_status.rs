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
pub struct KeypadStatus {
    /// Is device lost ? (Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "device_lost", skip_serializing_if = "Option::is_none")]
    pub device_lost: Option<bool>,
    /// Last status date (Box, AIO, AIO+, Pir, Siren, OutdoorSiren, Remote, Tag, Extender).
    #[serde(rename = "last_status_at", skip_serializing_if = "Option::is_none")]
    pub last_status_at: Option<String>,
}

impl KeypadStatus {
    pub fn new() -> KeypadStatus {
        KeypadStatus {
            device_lost: None,
            last_status_at: None,
        }
    }
}


