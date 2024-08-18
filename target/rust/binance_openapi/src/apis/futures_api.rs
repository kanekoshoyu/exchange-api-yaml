/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`sapi_v1_futures_hist_data_link_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1FuturesHistDataLinkGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_futures_transfer_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1FuturesTransferGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_futures_transfer_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1FuturesTransferPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Weight(IP): 1
pub async fn sapi_v1_futures_hist_data_link_get(configuration: &configuration::Configuration, symbol: &str, data_type: &str, timestamp: i64, signature: &str, start_time: Option<i64>, end_time: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1FuturesHistDataLinkGet200Response, Error<SapiV1FuturesHistDataLinkGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/futures/histDataLink", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("symbol", &symbol.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("dataType", &data_type.to_string())]);
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1FuturesHistDataLinkGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Weight(IP): 10
pub async fn sapi_v1_futures_transfer_get(configuration: &configuration::Configuration, asset: &str, start_time: i64, timestamp: i64, signature: &str, end_time: Option<i64>, current: Option<i32>, size: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1FuturesTransferGet200Response, Error<SapiV1FuturesTransferGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/futures/transfer", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("asset", &asset.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("startTime", &start_time.to_string())]);
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = current {
        local_var_req_builder = local_var_req_builder.query(&[("current", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = size {
        local_var_req_builder = local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1FuturesTransferGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Execute transfer between spot account and futures account.  Weight(IP): 1
pub async fn sapi_v1_futures_transfer_post(configuration: &configuration::Configuration, asset: &str, amount: f64, r#type: i64, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Response, Error<SapiV1FuturesTransferPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/futures/transfer", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("asset", &asset.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("amount", &amount.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("type", &r#type.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1FuturesTransferPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

