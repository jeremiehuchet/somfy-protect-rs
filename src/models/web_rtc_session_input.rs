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
pub struct WebRtcSessionInput {
    /// WebRTC Session Identifier.
    #[serde(rename = "session_id")]
    pub session_id: serde_json::Value,
}

impl WebRtcSessionInput {
    pub fn new(session_id: serde_json::Value) -> WebRtcSessionInput {
        WebRtcSessionInput {
            session_id,
        }
    }
}


