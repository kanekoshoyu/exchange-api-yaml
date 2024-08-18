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
pub struct MyTrade {
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Trade id
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    /// Price
    #[serde(rename = "price")]
    pub price: String,
    /// Amount of base asset
    #[serde(rename = "qty")]
    pub qty: String,
    /// Amount of quote asset
    #[serde(rename = "quoteQty")]
    pub quote_qty: String,
    #[serde(rename = "commission")]
    pub commission: String,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    /// Trade timestamp
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}

impl MyTrade {
    pub fn new(symbol: String, id: i64, order_id: i64, order_list_id: i64, price: String, qty: String, quote_qty: String, commission: String, commission_asset: String, time: i64, is_buyer: bool, is_maker: bool, is_best_match: bool) -> MyTrade {
        MyTrade {
            symbol,
            id,
            order_id,
            order_list_id,
            price,
            qty,
            quote_qty,
            commission,
            commission_asset,
            time,
            is_buyer,
            is_maker,
            is_best_match,
        }
    }
}

