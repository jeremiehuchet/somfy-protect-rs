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
pub struct ServiceCheckoutInput {
    /// Plan identifier to subscribe.
    #[serde(rename = "name")]
    pub name: String,
    /// Plan options.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::ServiceCheckoutInputOption>>,
}

impl ServiceCheckoutInput {
    pub fn new(name: String) -> ServiceCheckoutInput {
        ServiceCheckoutInput {
            name,
            options: None,
        }
    }
}


