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
pub struct UserGuestInput {
    /// Site identifier.
    #[serde(rename = "site_id", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// User display name on the site.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Identifier of user photo.
    #[serde(rename = "photo_id", skip_serializing_if = "Option::is_none")]
    pub photo_id: Option<String>,
    /// User firstname.
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// User lastname.
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// Profiles of user for this site.
    #[serde(rename = "profiles", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<crate::models::UserSiteProfile>>,
}

impl UserGuestInput {
    pub fn new() -> UserGuestInput {
        UserGuestInput {
            site_id: None,
            display_name: None,
            photo_id: None,
            firstname: None,
            lastname: None,
            profiles: None,
        }
    }
}


