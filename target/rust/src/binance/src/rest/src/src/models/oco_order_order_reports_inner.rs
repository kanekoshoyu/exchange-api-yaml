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
pub struct OcoOrderOrderReportsInner {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
}

impl OcoOrderOrderReportsInner {
    pub fn new(symbol: String, orig_client_order_id: String, order_id: i64, order_list_id: i64, client_order_id: String, price: String, orig_qty: String, executed_qty: String, cummulative_quote_qty: String, status: String, time_in_force: String, r#type: String, side: String, stop_price: String, self_trade_prevention_mode: String, transact_time: i64) -> OcoOrderOrderReportsInner {
        OcoOrderOrderReportsInner {
            symbol,
            orig_client_order_id,
            order_id,
            order_list_id,
            client_order_id,
            price,
            orig_qty,
            executed_qty,
            cummulative_quote_qty,
            status,
            time_in_force,
            r#type,
            side,
            stop_price,
            self_trade_prevention_mode,
            transact_time,
        }
    }
}

