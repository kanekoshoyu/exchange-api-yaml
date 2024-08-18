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
pub struct SapiV1MiningHashTransferConfigCancelPost200Response {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "data")]
    pub data: bool,
}

impl SapiV1MiningHashTransferConfigCancelPost200Response {
    pub fn new(code: i64, msg: String, data: bool) -> SapiV1MiningHashTransferConfigCancelPost200Response {
        SapiV1MiningHashTransferConfigCancelPost200Response {
            code,
            msg,
            data,
        }
    }
}

