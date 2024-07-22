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
pub struct DeviceCompatibility {
    #[serde(rename = "device")]
    pub device: Box<models::DeviceDefinition>,
    /// The number of installed devices
    #[serde(rename = "installed")]
    pub installed: serde_json::Value,
    /// The maximum of installable devices
    #[serde(rename = "max")]
    pub max: i32,
}

impl DeviceCompatibility {
    pub fn new(
        device: models::DeviceDefinition,
        installed: serde_json::Value,
        max: i32,
    ) -> DeviceCompatibility {
        DeviceCompatibility {
            device: Box::new(device),
            installed,
            max,
        }
    }
}
