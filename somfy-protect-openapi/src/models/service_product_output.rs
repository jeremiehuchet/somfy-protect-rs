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
pub struct ServiceProductOutput {
    #[serde(rename = "devices")]
    pub devices: Box<crate::models::DeviceOutputMinimal>,
    /// Currency code.
    #[serde(rename = "currency")]
    pub currency: String,
    /// List of prices.
    #[serde(rename = "prices")]
    pub prices: Vec<i32>,
    /// Product identifier.
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,
}

impl ServiceProductOutput {
    pub fn new(
        devices: crate::models::DeviceOutputMinimal,
        currency: String,
        prices: Vec<i32>,
    ) -> ServiceProductOutput {
        ServiceProductOutput {
            devices: Box::new(devices),
            currency,
            prices,
            product_id: None,
        }
    }
}

/// Product identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductId {
    #[serde(rename = "cvr1")]
    Cvr1,
    #[serde(rename = "cvr7")]
    Cvr7,
}

impl Default for ProductId {
    fn default() -> ProductId {
        Self::Cvr1
    }
}
