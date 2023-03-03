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
pub struct SiteAlarm {
    /// Site alarm status.
    #[serde(rename = "status")]
    pub status: Status,
    /// Type of last alarm.
    #[serde(rename = "alarm_type", skip_serializing_if = "Option::is_none")]
    pub alarm_type: Option<AlarmType>,
    /// Is last alarm manual or automatic.
    #[serde(rename = "manual_alarm", skip_serializing_if = "Option::is_none")]
    pub manual_alarm: Option<bool>,
    /// Starting time of last alarm.
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    /// Starting siren time of last alarm.
    #[serde(rename = "start_siren_at", skip_serializing_if = "Option::is_none")]
    pub start_siren_at: Option<String>,
    /// End siren time of last alarm.
    #[serde(rename = "end_siren_at", skip_serializing_if = "Option::is_none")]
    pub end_siren_at: Option<String>,
    /// End time of last alarm.
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    /// User identifier who stopped the last alarm.
    #[serde(rename = "stopped_by_user_id", skip_serializing_if = "Option::is_none")]
    pub stopped_by_user_id: Option<String>,
}

impl SiteAlarm {
    pub fn new(status: Status) -> SiteAlarm {
        SiteAlarm {
            status,
            alarm_type: None,
            manual_alarm: None,
            start_at: None,
            start_siren_at: None,
            end_siren_at: None,
            end_at: None,
            stopped_by_user_id: None,
        }
    }
}

/// Site alarm status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ongoing")]
    Ongoing,
}

impl Default for Status {
    fn default() -> Status {
        Self::None
    }
}
/// Type of last alarm.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlarmType {
    #[serde(rename = "panic")]
    Panic,
    #[serde(rename = "trespass")]
    Trespass,
    #[serde(rename = "smoke")]
    Smoke,
}

impl Default for AlarmType {
    fn default() -> AlarmType {
        Self::Panic
    }
}

