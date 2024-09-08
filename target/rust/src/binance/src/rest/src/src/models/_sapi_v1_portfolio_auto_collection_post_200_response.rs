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
pub struct SapiV1PortfolioAutoCollectionPost200Response {
    #[serde(rename = "msg")]
    pub msg: String,
}

impl SapiV1PortfolioAutoCollectionPost200Response {
    pub fn new(msg: String) -> SapiV1PortfolioAutoCollectionPost200Response {
        SapiV1PortfolioAutoCollectionPost200Response {
            msg,
        }
    }
}

