/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV3OrderPost200Response {
    OrderResponseAck(Box<models::OrderResponseAck>),
    OrderResponseResult(Box<models::OrderResponseResult>),
    OrderResponseFull(Box<models::OrderResponseFull>),
}

impl Default for ApiV3OrderPost200Response {
    fn default() -> Self {
        Self::OrderResponseAck(Default::default())
    }
}

