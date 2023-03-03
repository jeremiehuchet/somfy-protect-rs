/*
 * Somfy Protect API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`site_scenario_create`]
#[derive(Clone, Debug, Default)]
pub struct SiteScenarioCreateParams {
    /// Site identifier
    pub site_id: String,
    /// New scenario to create
    pub scenario: crate::models::ScenarioInput
}

/// struct for passing parameters to the method [`site_scenario_delete`]
#[derive(Clone, Debug, Default)]
pub struct SiteScenarioDeleteParams {
    /// Site identifier
    pub site_id: String,
    /// Scenario identifier
    pub scenario_id: String
}

/// struct for passing parameters to the method [`site_scenario_get_list`]
#[derive(Clone, Debug, Default)]
pub struct SiteScenarioGetListParams {
    /// Site identifier
    pub site_id: String
}

/// struct for passing parameters to the method [`site_scenario_update`]
#[derive(Clone, Debug, Default)]
pub struct SiteScenarioUpdateParams {
    /// Site identifier
    pub site_id: String,
    /// Scenario identifier
    pub scenario_id: String,
    /// New scenario data
    pub scenario: crate::models::ScenarioInput
}


/// struct for typed errors of method [`site_scenario_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteScenarioCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_scenario_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteScenarioDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_scenario_get_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteScenarioGetListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`site_scenario_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SiteScenarioUpdateError {
    UnknownValue(serde_json::Value),
}


pub async fn site_scenario_create(configuration: &configuration::Configuration, params: SiteScenarioCreateParams) -> Result<crate::models::Job, Error<SiteScenarioCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let scenario = params.scenario;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site/{site_id}/scenario", local_var_configuration.base_path, site_id=crate::apis::urlencode(site_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&scenario);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteScenarioCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_scenario_delete(configuration: &configuration::Configuration, params: SiteScenarioDeleteParams) -> Result<crate::models::Job, Error<SiteScenarioDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let scenario_id = params.scenario_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site/{site_id}/scenario/{scenario_id}", local_var_configuration.base_path, site_id=crate::apis::urlencode(site_id), scenario_id=crate::apis::urlencode(scenario_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteScenarioDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_scenario_get_list(configuration: &configuration::Configuration, params: SiteScenarioGetListParams) -> Result<crate::models::ScenarioCollection, Error<SiteScenarioGetListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site/{site_id}/scenario", local_var_configuration.base_path, site_id=crate::apis::urlencode(site_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteScenarioGetListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn site_scenario_update(configuration: &configuration::Configuration, params: SiteScenarioUpdateParams) -> Result<crate::models::Job, Error<SiteScenarioUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let site_id = params.site_id;
    let scenario_id = params.scenario_id;
    let scenario = params.scenario;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/site/{site_id}/scenario/{scenario_id}", local_var_configuration.base_path, site_id=crate::apis::urlencode(site_id), scenario_id=crate::apis::urlencode(scenario_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&scenario);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SiteScenarioUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

