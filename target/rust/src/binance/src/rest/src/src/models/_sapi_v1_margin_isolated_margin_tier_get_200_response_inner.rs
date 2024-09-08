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
pub struct SapiV1MarginIsolatedMarginTierGet200ResponseInner {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
    #[serde(rename = "effectiveMultiple", skip_serializing_if = "Option::is_none")]
    pub effective_multiple: Option<String>,
    #[serde(rename = "initialRiskRatio", skip_serializing_if = "Option::is_none")]
    pub initial_risk_ratio: Option<String>,
    #[serde(rename = "liquidationRiskRatio", skip_serializing_if = "Option::is_none")]
    pub liquidation_risk_ratio: Option<String>,
    #[serde(rename = "baseAssetMaxBorrowable", skip_serializing_if = "Option::is_none")]
    pub base_asset_max_borrowable: Option<String>,
    #[serde(rename = "quoteAssetMaxBorrowable", skip_serializing_if = "Option::is_none")]
    pub quote_asset_max_borrowable: Option<String>,
}

impl SapiV1MarginIsolatedMarginTierGet200ResponseInner {
    pub fn new() -> SapiV1MarginIsolatedMarginTierGet200ResponseInner {
        SapiV1MarginIsolatedMarginTierGet200ResponseInner {
            symbol: None,
            tier: None,
            effective_multiple: None,
            initial_risk_ratio: None,
            liquidation_risk_ratio: None,
            base_asset_max_borrowable: None,
            quote_asset_max_borrowable: None,
        }
    }
}

