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
pub struct ScenarioInput {
    /// Start time of Scenario in format HH:MM.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Days of week when Scenario must be applied. Possible values for days: mo, tu, we, th, fr, sa, su.
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<String>>,
    /// Security level.
    #[serde(rename = "security_level", skip_serializing_if = "Option::is_none")]
    pub security_level: Option<SecurityLevel>,
    /// Is scenario enabled?
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "scenario_action", skip_serializing_if = "Option::is_none")]
    pub scenario_action: Option<Box<crate::models::ScenarioAction>>,
}

impl ScenarioInput {
    pub fn new() -> ScenarioInput {
        ScenarioInput {
            time: None,
            days: None,
            security_level: None,
            enabled: None,
            scenario_action: None,
        }
    }
}

/// Security level.
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
