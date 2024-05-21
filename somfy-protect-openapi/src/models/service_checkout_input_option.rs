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
pub struct ServiceCheckoutInputOption {
    /// Option name.
    #[serde(rename = "name")]
    pub name: String,
    /// Devices list.
    #[serde(rename = "devices")]
    pub devices: Vec<crate::models::ServiceCheckoutInputOptionDevice>,
}

impl ServiceCheckoutInputOption {
    pub fn new(
        name: String,
        devices: Vec<crate::models::ServiceCheckoutInputOptionDevice>,
    ) -> ServiceCheckoutInputOption {
        ServiceCheckoutInputOption { name, devices }
    }
}
