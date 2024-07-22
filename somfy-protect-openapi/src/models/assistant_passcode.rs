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
pub struct AssistantPasscode {
    /// Four digit passcode for alexa.
    #[serde(rename = "passcode")]
    pub passcode: String,
    /// Previous passcode for alexa.
    #[serde(rename = "old_passcode", skip_serializing_if = "Option::is_none")]
    pub old_passcode: Option<String>,
}

impl AssistantPasscode {
    pub fn new(passcode: String) -> AssistantPasscode {
        AssistantPasscode {
            passcode,
            old_passcode: None,
        }
    }
}
