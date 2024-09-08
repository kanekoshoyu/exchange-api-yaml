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
pub struct SapiV1LoanAdjustLtvPost200Response {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "currentLTV")]
    pub current_ltv: String,
}

impl SapiV1LoanAdjustLtvPost200Response {
    pub fn new(loan_coin: String, collateral_coin: String, direction: String, amount: String, current_ltv: String) -> SapiV1LoanAdjustLtvPost200Response {
        SapiV1LoanAdjustLtvPost200Response {
            loan_coin,
            collateral_coin,
            direction,
            amount,
            current_ltv,
        }
    }
}

