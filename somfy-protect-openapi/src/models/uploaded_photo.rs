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
pub struct UploadedPhoto {
    /// The photo identifier.
    #[serde(rename = "photo_id")]
    pub photo_id: String,
}

impl UploadedPhoto {
    pub fn new(photo_id: String) -> UploadedPhoto {
        UploadedPhoto { photo_id }
    }
}
