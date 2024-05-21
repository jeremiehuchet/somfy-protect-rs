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
pub struct ServiceDevice {
    /// Device identifier.
    #[serde(rename = "device_id")]
    pub device_id: String,
    /// Device label.
    #[serde(rename = "label")]
    pub label: String,
}

impl ServiceDevice {
    pub fn new(device_id: String, label: String) -> ServiceDevice {
        ServiceDevice { device_id, label }
    }
}
