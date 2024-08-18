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
pub struct SapiV2LoanFlexibleCollateralDataGet200ResponseRowsInner {
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "initialLTV")]
    pub initial_ltv: String,
    #[serde(rename = "marginCallLTV")]
    pub margin_call_ltv: String,
    #[serde(rename = "liquidationLTV")]
    pub liquidation_ltv: String,
    #[serde(rename = "maxLimit")]
    pub max_limit: String,
}

impl SapiV2LoanFlexibleCollateralDataGet200ResponseRowsInner {
    pub fn new(collateral_coin: String, initial_ltv: String, margin_call_ltv: String, liquidation_ltv: String, max_limit: String) -> SapiV2LoanFlexibleCollateralDataGet200ResponseRowsInner {
        SapiV2LoanFlexibleCollateralDataGet200ResponseRowsInner {
            collateral_coin,
            initial_ltv,
            margin_call_ltv,
            liquidation_ltv,
            max_limit,
        }
    }
}

