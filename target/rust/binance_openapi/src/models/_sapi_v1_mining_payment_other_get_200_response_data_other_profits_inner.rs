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
pub struct SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner {
    /// Mining date
    #[serde(rename = "time")]
    pub time: i64,
    /// Coin Name
    #[serde(rename = "coinName")]
    pub coin_name: String,
    /// 1: Merged Mining, 2: Activity Bonus, 3:Rebate 4:Smart Pool 6:Income Transfer 7:Pool Savings
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "profitAmount")]
    pub profit_amount: f64,
    /// 0:Unpaid, 1:Paying  2：Paid
    #[serde(rename = "status")]
    pub status: i32,
}

impl SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner {
    pub fn new(time: i64, coin_name: String, r#type: i32, profit_amount: f64, status: i32) -> SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner {
        SapiV1MiningPaymentOtherGet200ResponseDataOtherProfitsInner {
            time,
            coin_name,
            r#type,
            profit_amount,
            status,
        }
    }
}

