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
pub struct UserInputWithSecureToken {
    /// User activation status.
    #[serde(rename = "activated", skip_serializing_if = "Option::is_none")]
    pub activated: Option<bool>,
    /// User password (minimum 8 characters).
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// User firstname.
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// User lastname.
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// User email.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// User gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// User locale.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// User phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UserInputWithSecureToken {
    pub fn new() -> UserInputWithSecureToken {
        UserInputWithSecureToken {
            activated: None,
            password: None,
            firstname: None,
            lastname: None,
            username: None,
            gender: None,
            locale: None,
            phone: None,
        }
    }
}

/// User gender.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "F")]
    F,
    #[serde(rename = "M")]
    M,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::F
    }
}
