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
pub struct SapiV1MiningPaymentUidGet200Response {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "data")]
    pub data: Box<models::SapiV1MiningPaymentUidGet200ResponseData>,
}

impl SapiV1MiningPaymentUidGet200Response {
    pub fn new(code: i32, msg: String, data: models::SapiV1MiningPaymentUidGet200ResponseData) -> SapiV1MiningPaymentUidGet200Response {
        SapiV1MiningPaymentUidGet200Response {
            code,
            msg,
            data: Box::new(data),
        }
    }
}

