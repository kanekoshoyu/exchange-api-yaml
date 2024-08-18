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
pub struct SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner {
    #[serde(rename = "targetAsset")]
    pub target_asset: String,
    #[serde(rename = "averagePriceInUSD")]
    pub average_price_in_usd: String,
    #[serde(rename = "totalInvestedInUSD")]
    pub total_invested_in_usd: String,
    #[serde(rename = "purchasedAmount")]
    pub purchased_amount: String,
    #[serde(rename = "purchasedAmountUnit")]
    pub purchased_amount_unit: String,
    #[serde(rename = "pnlInUSD")]
    pub pnl_in_usd: String,
    #[serde(rename = "roi")]
    pub roi: String,
    #[serde(rename = "percentage")]
    pub percentage: String,
    #[serde(rename = "assetStatus")]
    pub asset_status: String,
    #[serde(rename = "availableAmount")]
    pub available_amount: String,
    #[serde(rename = "availableAmountUnit")]
    pub available_amount_unit: String,
    #[serde(rename = "redeemedAmout")]
    pub redeemed_amout: String,
    #[serde(rename = "redeemedAmoutUnit")]
    pub redeemed_amout_unit: String,
    #[serde(rename = "assetValueInUSD")]
    pub asset_value_in_usd: String,
}

impl SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner {
    pub fn new(target_asset: String, average_price_in_usd: String, total_invested_in_usd: String, purchased_amount: String, purchased_amount_unit: String, pnl_in_usd: String, roi: String, percentage: String, asset_status: String, available_amount: String, available_amount_unit: String, redeemed_amout: String, redeemed_amout_unit: String, asset_value_in_usd: String) -> SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner {
        SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner {
            target_asset,
            average_price_in_usd,
            total_invested_in_usd,
            purchased_amount,
            purchased_amount_unit,
            pnl_in_usd,
            roi,
            percentage,
            asset_status,
            available_amount,
            available_amount_unit,
            redeemed_amout,
            redeemed_amout_unit,
            asset_value_in_usd,
        }
    }
}

