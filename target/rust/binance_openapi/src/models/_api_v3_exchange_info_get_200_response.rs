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
pub struct ApiV3ExchangeInfoGet200Response {
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<models::ApiV3ExchangeInfoGet200ResponseRateLimitsInner>,
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<serde_json::Value>,
    #[serde(rename = "symbols")]
    pub symbols: Vec<models::ApiV3ExchangeInfoGet200ResponseSymbolsInner>,
}

impl ApiV3ExchangeInfoGet200Response {
    pub fn new(timezone: String, server_time: i64, rate_limits: Vec<models::ApiV3ExchangeInfoGet200ResponseRateLimitsInner>, exchange_filters: Vec<serde_json::Value>, symbols: Vec<models::ApiV3ExchangeInfoGet200ResponseSymbolsInner>) -> ApiV3ExchangeInfoGet200Response {
        ApiV3ExchangeInfoGet200Response {
            timezone,
            server_time,
            rate_limits,
            exchange_filters,
            symbols,
        }
    }
}

