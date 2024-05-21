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
pub struct DeviceAbstract {
    /// The device label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The room id.
    #[serde(rename = "roomId", skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::DeviceSettings>>,
}

impl DeviceAbstract {
    pub fn new() -> DeviceAbstract {
        DeviceAbstract {
            label: None,
            room_id: None,
            settings: None,
        }
    }
}
