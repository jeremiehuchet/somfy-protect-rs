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
pub struct SiteDiagnosisItem {
    #[serde(rename = "device_id", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "problem", skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
}

impl SiteDiagnosisItem {
    pub fn new() -> SiteDiagnosisItem {
        SiteDiagnosisItem {
            device_id: None,
            problem: None,
        }
    }
}


