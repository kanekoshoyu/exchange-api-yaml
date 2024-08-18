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
pub struct SapiV2LoanFlexibleBorrowPost200Response {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "loanAmount")]
    pub loan_amount: String,
    #[serde(rename = "collateralCoin", skip_serializing_if = "Option::is_none")]
    pub collateral_coin: Option<String>,
    #[serde(rename = "collateralAmount")]
    pub collateral_amount: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV2LoanFlexibleBorrowPost200Response {
    pub fn new(loan_coin: String, loan_amount: String, collateral_amount: String, status: String) -> SapiV2LoanFlexibleBorrowPost200Response {
        SapiV2LoanFlexibleBorrowPost200Response {
            loan_coin,
            loan_amount,
            collateral_coin: None,
            collateral_amount,
            status,
        }
    }
}

