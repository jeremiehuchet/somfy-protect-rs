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
pub struct DeviceSettings {
    #[serde(rename = "global")]
    pub global: Box<crate::models::DeviceSettingsGlobal>,
    #[serde(rename = "disarmed", skip_serializing_if = "Option::is_none")]
    pub disarmed: Option<Box<crate::models::DeviceSettingsDisarmed>>,
    #[serde(rename = "partial", skip_serializing_if = "Option::is_none")]
    pub partial: Option<Box<crate::models::DeviceSettingsPartial>>,
    #[serde(rename = "armed", skip_serializing_if = "Option::is_none")]
    pub armed: Option<Box<crate::models::DeviceSettingsArmed>>,
}

impl DeviceSettings {
    pub fn new(global: crate::models::DeviceSettingsGlobal) -> DeviceSettings {
        DeviceSettings {
            global: Box::new(global),
            disarmed: None,
            partial: None,
            armed: None,
        }
    }
}
