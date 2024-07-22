/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`site_invitation_answer`]
#[derive(Clone, Debug)]
pub struct SiteInvitationAnswerParams {
    /// Security token
    pub security_token: String,
    /// Invitation answer details
    pub answer: models::SiteInvitationAnswer,
}

/// struct for passing parameters to the method [`site_invitation_send`]
#[derive(Clone, Debug)]
pub struct SiteInvitationSendParams {
    /// Site identifier
    pub site_id: String,
    /// Invitation details
    pub invitation: models::SiteInvitationRequest,
}

/// struct for typed errors of method [`site_invitation_answer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteInvitationAnswerError {
    Status400(),
    Status404(models::ApiException),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_invitation_send`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteInvitationSendError {
    Status404(models::ApiException),
    Status400(models::ApiException),
    UnknownValue(serde_json::Value),
}

pub async fn site_invitation_answer(
    configuration: &configuration::Configuration,
    params: SiteInvitationAnswerParams,
) -> Result<(), Error<SiteInvitationAnswerError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let security_token = params.security_token;
    let answer = params.answer;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site/invitation", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder =
        local_var_req_builder.query(&[("security_token", &security_token.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&answer);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SiteInvitationAnswerError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_invitation_send(
    configuration: &configuration::Configuration,
    params: SiteInvitationSendParams,
) -> Result<models::UserOutput, Error<SiteInvitationSendError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let invitation = params.invitation;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/site/{site_id}/invitation",
        local_var_configuration.base_path,
        site_id = crate::apis::urlencode(site_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&invitation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteInvitationSendError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
