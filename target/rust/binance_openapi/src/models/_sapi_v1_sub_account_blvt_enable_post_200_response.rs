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
pub struct SapiV1SubAccountBlvtEnablePost200Response {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "enableBlvt")]
    pub enable_blvt: bool,
}

impl SapiV1SubAccountBlvtEnablePost200Response {
    pub fn new(email: String, enable_blvt: bool) -> SapiV1SubAccountBlvtEnablePost200Response {
        SapiV1SubAccountBlvtEnablePost200Response {
            email,
            enable_blvt,
        }
    }
}

