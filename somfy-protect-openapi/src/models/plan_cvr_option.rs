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
pub struct PlanCvrOption {
    /// Selected.
    #[serde(rename = "selected")]
    pub selected: bool,
    /// CVR Span 1|7.
    #[serde(rename = "cvr_span")]
    pub cvr_span: i32,
    /// First camera price.
    #[serde(rename = "first_camera_price")]
    pub first_camera_price: Vec<models::PlanPriceCamera>,
    /// Other camera price.
    #[serde(rename = "other_camera_price")]
    pub other_camera_price: Vec<models::PlanPriceCamera>,
}

impl PlanCvrOption {
    pub fn new(
        selected: bool,
        cvr_span: i32,
        first_camera_price: Vec<models::PlanPriceCamera>,
        other_camera_price: Vec<models::PlanPriceCamera>,
    ) -> PlanCvrOption {
        PlanCvrOption {
            selected,
            cvr_span,
            first_camera_price,
            other_camera_price,
        }
    }
}
