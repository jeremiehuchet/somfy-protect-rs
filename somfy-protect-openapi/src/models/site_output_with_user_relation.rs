/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteOutputWithUserRelation {
    /// Profiles of user for this site.
    #[serde(rename = "profiles", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<models::UserSiteProfile>>,
    /// Camera token.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Short site identifier.
    #[serde(rename = "short_site_id", skip_serializing_if = "Option::is_none")]
    pub short_site_id: Option<String>,
    #[serde(rename = "geoFence", skip_serializing_if = "Option::is_none")]
    pub geo_fence: Option<Box<models::GeoFenceOutput>>,
    /// Alexa selection datetime.
    #[serde(rename = "alexa_selected_at", skip_serializing_if = "Option::is_none")]
    pub alexa_selected_at: Option<String>,
    /// Alexa configuration state.
    #[serde(rename = "alexa_state", skip_serializing_if = "Option::is_none")]
    pub alexa_state: Option<String>,
    /// Alexa availability on the site.
    #[serde(rename = "alexa_available", skip_serializing_if = "Option::is_none")]
    pub alexa_available: Option<bool>,
    /// Status of presence's user.
    #[serde(
        rename = "display_my_presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_my_presence: Option<bool>,
    /// Site identifier.
    #[serde(rename = "site_id", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// Site security level.
    #[serde(rename = "security_level", skip_serializing_if = "Option::is_none")]
    pub security_level: Option<SecurityLevel>,
    #[serde(rename = "invitation", skip_serializing_if = "Option::is_none")]
    pub invitation: Option<Box<models::SiteInvitationOutput>>,
    #[serde(rename = "alarm", skip_serializing_if = "Option::is_none")]
    pub alarm: Option<Box<models::SiteAlarm>>,
    #[serde(rename = "diagnosis", skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Box<models::SiteDiagnosis>>,
    /// Install completed flag.
    #[serde(rename = "install_completed", skip_serializing_if = "Option::is_none")]
    pub install_completed: Option<bool>,
    /// Brand.
    #[serde(rename = "brand", skip_serializing_if = "Option::is_none")]
    pub brand: Option<serde_json::Value>,
    /// Is a Plug configuration currently being backed up?
    #[serde(rename = "restoration_active", skip_serializing_if = "Option::is_none")]
    pub restoration_active: Option<bool>,
    /// Privacy current status.
    #[serde(rename = "privacy_active", skip_serializing_if = "Option::is_none")]
    pub privacy_active: Option<bool>,
    /// Read only status.
    #[serde(rename = "read_only", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// HCS site identifier.
    #[serde(rename = "hcs_site_id", skip_serializing_if = "Option::is_none")]
    pub hcs_site_id: Option<String>,
    /// ILO site identifier.
    #[serde(rename = "ilo_site_id", skip_serializing_if = "Option::is_none")]
    pub ilo_site_id: Option<String>,
    /// Site origin.
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Axa IT phone number.
    #[serde(
        rename = "axa_it_phone_number",
        skip_serializing_if = "Option::is_none"
    )]
    pub axa_it_phone_number: Option<String>,
    /// Axa IT phone number formatted.
    #[serde(
        rename = "axa_it_phone_number_displayed",
        skip_serializing_if = "Option::is_none"
    )]
    pub axa_it_phone_number_displayed: Option<String>,
    /// Features enabled.
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(
        rename = "domesticAlarmOutputCollection",
        skip_serializing_if = "Option::is_none"
    )]
    pub domestic_alarm_output_collection: Option<Box<models::DomesticAlarmOutputCollection>>,
    /// Bundle type for  ILO.
    #[serde(rename = "bundle_type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<BundleType>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Box<models::SiteServicesOutput>>,
    /// Site label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
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
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
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

impl SiteOutputWithUserRelation {
    pub fn new() -> SiteOutputWithUserRelation {
        SiteOutputWithUserRelation {
            profiles: None,
            token: None,
            short_site_id: None,
            geo_fence: None,
            alexa_selected_at: None,
            alexa_state: None,
            alexa_available: None,
            display_my_presence: None,
            site_id: None,
            security_level: None,
            invitation: None,
            alarm: None,
            diagnosis: None,
            install_completed: None,
            brand: None,
            restoration_active: None,
            privacy_active: None,
            read_only: None,
            hcs_site_id: None,
            ilo_site_id: None,
            origin: None,
            axa_it_phone_number: None,
            axa_it_phone_number_displayed: None,
            features: None,
            domestic_alarm_output_collection: None,
            bundle_type: None,
            services: None,
            label: None,
            services_version: None,
            subscription_active: None,
            timezone: None,
            name: None,
            address1: None,
            address2: None,
            zip_code: None,
            city: None,
            region: None,
            country: None,
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
/// Site security level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecurityLevel {
    #[serde(rename = "disarmed")]
    Disarmed,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "armed")]
    Armed,
}

impl Default for SecurityLevel {
    fn default() -> SecurityLevel {
        Self::Disarmed
    }
}
/// Bundle type for  ILO.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BundleType {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "flat")]
    Flat,
}

impl Default for BundleType {
    fn default() -> BundleType {
        Self::Home
    }
}
