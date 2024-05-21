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
pub struct ServiceStatus {
    /// Service provider.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Service is displayed.
    #[serde(rename = "displayed")]
    pub displayed: bool,
    /// Service is available.
    #[serde(rename = "available")]
    pub available: bool,
    /// Service has warning.
    #[serde(rename = "warning")]
    pub warning: bool,
    /// Service status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Service link to next plan.
    #[serde(rename = "linkedCode", skip_serializing_if = "Option::is_none")]
    pub linked_code: Option<String>,
}

impl ServiceStatus {
    pub fn new(provider: String, displayed: bool, available: bool, warning: bool) -> ServiceStatus {
        ServiceStatus {
            provider,
            displayed,
            available,
            warning,
            status: None,
            linked_code: None,
        }
    }
}
