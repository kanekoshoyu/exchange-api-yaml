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
pub struct MarginTrade {
    #[serde(rename = "commission")]
    pub commission: String,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    #[serde(rename = "time")]
    pub time: i64,
}

impl MarginTrade {
    pub fn new(commission: String, commission_asset: String, id: i64, is_best_match: bool, is_buyer: bool, is_maker: bool, order_id: i64, price: String, qty: String, symbol: String, is_isolated: bool, time: i64) -> MarginTrade {
        MarginTrade {
            commission,
            commission_asset,
            id,
            is_best_match,
            is_buyer,
            is_maker,
            order_id,
            price,
            qty,
            symbol,
            is_isolated,
            time,
        }
    }
}

