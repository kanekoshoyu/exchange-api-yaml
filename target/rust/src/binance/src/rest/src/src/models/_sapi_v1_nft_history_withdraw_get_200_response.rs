/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1NftHistoryWithdrawGet200Response {
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "list")]
    pub list: Vec<models::SapiV1NftHistoryWithdrawGet200ResponseListInner>,
}

impl SapiV1NftHistoryWithdrawGet200Response {
    pub fn new(total: i32, list: Vec<models::SapiV1NftHistoryWithdrawGet200ResponseListInner>) -> SapiV1NftHistoryWithdrawGet200Response {
        SapiV1NftHistoryWithdrawGet200Response {
            total,
            list,
        }
    }
}

