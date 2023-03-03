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
pub struct SiteInstallerDeleteRequest {
    /// The password to check
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl SiteInstallerDeleteRequest {
    pub fn new() -> SiteInstallerDeleteRequest {
        SiteInstallerDeleteRequest {
            password: None,
        }
    }
}


