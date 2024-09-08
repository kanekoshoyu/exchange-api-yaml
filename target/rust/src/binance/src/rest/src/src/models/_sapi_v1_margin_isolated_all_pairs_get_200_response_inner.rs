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
pub struct SapiV1MarginIsolatedAllPairsGet200ResponseInner {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "base")]
    pub base: String,
    #[serde(rename = "quote")]
    pub quote: String,
    #[serde(rename = "isMarginTrade")]
    pub is_margin_trade: bool,
    #[serde(rename = "isBuyAllowed")]
    pub is_buy_allowed: bool,
    #[serde(rename = "isSellAllowed")]
    pub is_sell_allowed: bool,
}

impl SapiV1MarginIsolatedAllPairsGet200ResponseInner {
    pub fn new(symbol: String, base: String, quote: String, is_margin_trade: bool, is_buy_allowed: bool, is_sell_allowed: bool) -> SapiV1MarginIsolatedAllPairsGet200ResponseInner {
        SapiV1MarginIsolatedAllPairsGet200ResponseInner {
            symbol,
            base,
            quote,
            is_margin_trade,
            is_buy_allowed,
            is_sell_allowed,
        }
    }
}

