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
pub struct ApiV3RateLimitOrderGet200ResponseInner {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "intervalNum")]
    pub interval_num: i32,
    #[serde(rename = "limit")]
    pub limit: i32,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl ApiV3RateLimitOrderGet200ResponseInner {
    pub fn new(rate_limit_type: String, interval: String, interval_num: i32, limit: i32) -> ApiV3RateLimitOrderGet200ResponseInner {
        ApiV3RateLimitOrderGet200ResponseInner {
            rate_limit_type,
            interval,
            interval_num,
            limit,
            count: None,
        }
    }
}

