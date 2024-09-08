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
pub struct SapiV2LoanFlexibleRepayPost200Response {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "remainingDebt")]
    pub remaining_debt: String,
    #[serde(rename = "remainingCollateral")]
    pub remaining_collateral: String,
    #[serde(rename = "fullRepayment")]
    pub full_repayment: bool,
    #[serde(rename = "currentLTV")]
    pub current_ltv: String,
    /// Repaid, Repaying, Failed
    #[serde(rename = "repayStatus")]
    pub repay_status: String,
}

impl SapiV2LoanFlexibleRepayPost200Response {
    pub fn new(loan_coin: String, collateral_coin: String, remaining_debt: String, remaining_collateral: String, full_repayment: bool, current_ltv: String, repay_status: String) -> SapiV2LoanFlexibleRepayPost200Response {
        SapiV2LoanFlexibleRepayPost200Response {
            loan_coin,
            collateral_coin,
            remaining_debt,
            remaining_collateral,
            full_repayment,
            current_ltv,
            repay_status,
        }
    }
}

