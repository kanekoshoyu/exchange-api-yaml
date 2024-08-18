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
pub struct Ticker {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "priceChange")]
    pub price_change: String,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: String,
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    #[serde(rename = "bidPrice")]
    pub bid_price: String,
    #[serde(rename = "bidQty")]
    pub bid_qty: String,
    #[serde(rename = "askPrice")]
    pub ask_price: String,
    #[serde(rename = "askQty")]
    pub ask_qty: String,
    #[serde(rename = "openPrice")]
    pub open_price: String,
    #[serde(rename = "highPrice")]
    pub high_price: String,
    #[serde(rename = "lowPrice")]
    pub low_price: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: String,
    #[serde(rename = "openTime")]
    pub open_time: i64,
    #[serde(rename = "closeTime")]
    pub close_time: i64,
    #[serde(rename = "firstId")]
    pub first_id: i64,
    #[serde(rename = "lastId")]
    pub last_id: i64,
    #[serde(rename = "count")]
    pub count: i64,
}

impl Ticker {
    pub fn new(symbol: String, price_change: String, price_change_percent: String, prev_close_price: String, last_price: String, bid_price: String, bid_qty: String, ask_price: String, ask_qty: String, open_price: String, high_price: String, low_price: String, volume: String, quote_volume: String, open_time: i64, close_time: i64, first_id: i64, last_id: i64, count: i64) -> Ticker {
        Ticker {
            symbol,
            price_change,
            price_change_percent,
            prev_close_price,
            last_price,
            bid_price,
            bid_qty,
            ask_price,
            ask_qty,
            open_price,
            high_price,
            low_price,
            volume,
            quote_volume,
            open_time,
            close_time,
            first_id,
            last_id,
            count,
        }
    }
}

