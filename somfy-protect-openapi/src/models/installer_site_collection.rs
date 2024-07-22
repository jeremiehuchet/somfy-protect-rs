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
pub struct InstallerSiteCollection {
    /// Output[] List of sites.
    #[serde(rename = "items")]
    pub items: Vec<models::SiteOutput>,
}

impl InstallerSiteCollection {
    pub fn new(items: Vec<models::SiteOutput>) -> InstallerSiteCollection {
        InstallerSiteCollection { items }
    }
}
