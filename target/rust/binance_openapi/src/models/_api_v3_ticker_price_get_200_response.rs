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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV3TickerPriceGet200Response {
    PriceTicker(Box<models::PriceTicker>),
    PriceTickerList(Vec<models::PriceTicker>),
}

impl Default for ApiV3TickerPriceGet200Response {
    fn default() -> Self {
        Self::PriceTicker(Default::default())
    }
}

