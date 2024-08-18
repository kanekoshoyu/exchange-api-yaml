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
pub struct SapiV1GiftcardBuyCodeTokenLimitGet200Response {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "data")]
    pub data: Box<models::SapiV1GiftcardBuyCodeTokenLimitGet200ResponseData>,
    #[serde(rename = "success")]
    pub success: bool,
}

impl SapiV1GiftcardBuyCodeTokenLimitGet200Response {
    pub fn new(code: String, message: String, data: models::SapiV1GiftcardBuyCodeTokenLimitGet200ResponseData, success: bool) -> SapiV1GiftcardBuyCodeTokenLimitGet200Response {
        SapiV1GiftcardBuyCodeTokenLimitGet200Response {
            code,
            message,
            data: Box::new(data),
            success,
        }
    }
}

