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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1AccountInfoGet200Response {
    #[serde(rename = "vipLevel")]
    pub vip_level: i32,
    /// true or false for margin.
    #[serde(rename = "isMarginEnabled")]
    pub is_margin_enabled: bool,
    /// true or false for futures.
    #[serde(rename = "isFutureEnabled")]
    pub is_future_enabled: bool,
}

impl SapiV1AccountInfoGet200Response {
    pub fn new(vip_level: i32, is_margin_enabled: bool, is_future_enabled: bool) -> SapiV1AccountInfoGet200Response {
        SapiV1AccountInfoGet200Response {
            vip_level,
            is_margin_enabled,
            is_future_enabled,
        }
    }
}

