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
pub struct InstallOptionsHomeKit {
    /// MAC address when installing a device.
    #[serde(rename = "mac", skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// UUID token.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl InstallOptionsHomeKit {
    pub fn new() -> InstallOptionsHomeKit {
        InstallOptionsHomeKit {
            mac: None,
            uuid: None,
        }
    }
}
