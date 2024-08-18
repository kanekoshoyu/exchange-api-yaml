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
pub struct SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner {
    #[serde(rename = "algoId")]
    pub algo_id: i64,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "executedQty", skip_serializing_if = "Option::is_none")]
    pub executed_qty: Option<String>,
    #[serde(rename = "executedAmt")]
    pub executed_amt: String,
    #[serde(rename = "feeAmt")]
    pub fee_amt: String,
    #[serde(rename = "feeAsset")]
    pub fee_asset: String,
    #[serde(rename = "bookTime")]
    pub book_time: i64,
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "subId")]
    pub sub_id: i64,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
}

impl SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner {
    pub fn new(algo_id: i64, order_id: i64, order_status: String, executed_amt: String, fee_amt: String, fee_asset: String, book_time: i64, avg_price: String, side: String, symbol: String, sub_id: i64, time_in_force: String, orig_qty: String) -> SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner {
        SapiV1AlgoFuturesSubOrdersGet200ResponseSubOrdersInner {
            algo_id,
            order_id,
            order_status,
            executed_qty: None,
            executed_amt,
            fee_amt,
            fee_asset,
            book_time,
            avg_price,
            side,
            symbol,
            sub_id,
            time_in_force,
            orig_qty,
        }
    }
}

