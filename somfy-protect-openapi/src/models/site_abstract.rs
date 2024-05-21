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
pub struct SiteAbstract {
    /// Site label.
    #[serde(rename = "label")]
    pub label: String,
    /// Site Services Version.
    #[serde(rename = "servicesVersion", skip_serializing_if = "Option::is_none")]
    pub services_version: Option<i32>,
    /// Is site subscription active.
    #[serde(
        rename = "subscription_active",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_active: Option<bool>,
    /// Timezone of the site location.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// Name of the site location.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Address of the site location.
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// Address complement of the site location.
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// Post code of the site location.
    #[serde(rename = "zip_code", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// City of the site location.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Region/State of the site location.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Country code of the site location (uppercase two-letter ISO-3166-1 alpha-2 code).
    #[serde(rename = "country")]
    pub country: String,
    /// Cross street details.
    #[serde(rename = "crosstreet", skip_serializing_if = "Option::is_none")]
    pub crosstreet: Option<String>,
    /// Address complement.
    #[serde(rename = "complement", skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    /// Address latitude.
    #[serde(rename = "latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    /// Address longitude.
    #[serde(rename = "longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    /// Police phone number of the site area.
    #[serde(
        rename = "police_phone_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub police_phone_number: Option<String>,
    /// Enable KIDS feature.
    #[serde(rename = "kids_enabled", skip_serializing_if = "Option::is_none")]
    pub kids_enabled: Option<bool>,
    /// Display presence for Kids.
    #[serde(
        rename = "display_kid_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_kid_presence: Option<bool>,
    /// Display presence for guests.
    #[serde(
        rename = "display_guest_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_guest_presence: Option<bool>,
    /// Enable automatic shutter (for cameras).
    #[serde(
        rename = "shutter_automatic_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub shutter_automatic_enabled: Option<bool>,
    /// Enable user presence display.
    #[serde(rename = "presence_enabled", skip_serializing_if = "Option::is_none")]
    pub presence_enabled: Option<bool>,
    /// Disabled surveillance on disarm (for cameras).
    #[serde(
        rename = "outdoor_shutter_automatic_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub outdoor_shutter_automatic_enabled: Option<bool>,
    /// Enable smoke alarm detection.
    #[serde(
        rename = "smoke_alarm_listening_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub smoke_alarm_listening_enabled: Option<bool>,
    /// Enable Myfox Around.
    #[serde(rename = "mfa_enabled", skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    /// Myfox Around  Feature.
    #[serde(rename = "mfa_available", skip_serializing_if = "Option::is_none")]
    pub mfa_available: Option<bool>,
    #[serde(rename = "exitDelay", skip_serializing_if = "Option::is_none")]
    pub exit_delay: Option<i32>,
    /// Installer Id.
    #[serde(rename = "installer_id", skip_serializing_if = "Option::is_none")]
    pub installer_id: Option<String>,
}

impl SiteAbstract {
    pub fn new(label: String, timezone: String, country: String) -> SiteAbstract {
        SiteAbstract {
            label,
            services_version: None,
            subscription_active: None,
            timezone,
            name: None,
            address1: None,
            address2: None,
            zip_code: None,
            city: None,
            region: None,
            country,
            crosstreet: None,
            complement: None,
            latitude: None,
            longitude: None,
            police_phone_number: None,
            kids_enabled: None,
            display_kid_presence: None,
            display_guest_presence: None,
            shutter_automatic_enabled: None,
            presence_enabled: None,
            outdoor_shutter_automatic_enabled: None,
            smoke_alarm_listening_enabled: None,
            mfa_enabled: None,
            mfa_available: None,
            exit_delay: None,
            installer_id: None,
        }
    }
}
