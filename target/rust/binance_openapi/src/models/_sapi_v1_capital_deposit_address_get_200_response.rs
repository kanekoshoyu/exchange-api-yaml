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
pub struct SapiV1CapitalDepositAddressGet200Response {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "tag")]
    pub tag: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl SapiV1CapitalDepositAddressGet200Response {
    pub fn new(address: String, coin: String, tag: String, url: String) -> SapiV1CapitalDepositAddressGet200Response {
        SapiV1CapitalDepositAddressGet200Response {
            address,
            coin,
            tag,
            url,
        }
    }
}

