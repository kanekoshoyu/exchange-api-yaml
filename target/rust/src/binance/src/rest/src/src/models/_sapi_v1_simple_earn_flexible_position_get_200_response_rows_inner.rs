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
pub struct SapiV1SimpleEarnFlexiblePositionGet200ResponseRowsInner {
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    #[serde(rename = "tierAnnualPercentageRate")]
    pub tier_annual_percentage_rate: Box<models::SapiV1SimpleEarnFlexibleListGet200ResponseRowsInnerTierAnnualPercentageRate>,
    #[serde(rename = "latestAnnualPercentageRate")]
    pub latest_annual_percentage_rate: String,
    #[serde(rename = "yesterdayAirdropPercentageRate")]
    pub yesterday_airdrop_percentage_rate: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "airDropAsset")]
    pub air_drop_asset: String,
    #[serde(rename = "canRedeem")]
    pub can_redeem: bool,
    #[serde(rename = "collateralAmount")]
    pub collateral_amount: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "yesterdayRealTimeRewards")]
    pub yesterday_real_time_rewards: String,
    #[serde(rename = "cumulativeBonusRewards")]
    pub cumulative_bonus_rewards: String,
    #[serde(rename = "cumulativeRealTimeRewards")]
    pub cumulative_real_time_rewards: String,
    #[serde(rename = "cumulativeTotalRewards")]
    pub cumulative_total_rewards: String,
    #[serde(rename = "autoSubscribe")]
    pub auto_subscribe: bool,
}

impl SapiV1SimpleEarnFlexiblePositionGet200ResponseRowsInner {
    pub fn new(total_amount: String, tier_annual_percentage_rate: models::SapiV1SimpleEarnFlexibleListGet200ResponseRowsInnerTierAnnualPercentageRate, latest_annual_percentage_rate: String, yesterday_airdrop_percentage_rate: String, asset: String, air_drop_asset: String, can_redeem: bool, collateral_amount: String, product_id: String, yesterday_real_time_rewards: String, cumulative_bonus_rewards: String, cumulative_real_time_rewards: String, cumulative_total_rewards: String, auto_subscribe: bool) -> SapiV1SimpleEarnFlexiblePositionGet200ResponseRowsInner {
        SapiV1SimpleEarnFlexiblePositionGet200ResponseRowsInner {
            total_amount,
            tier_annual_percentage_rate: Box::new(tier_annual_percentage_rate),
            latest_annual_percentage_rate,
            yesterday_airdrop_percentage_rate,
            asset,
            air_drop_asset,
            can_redeem,
            collateral_amount,
            product_id,
            yesterday_real_time_rewards,
            cumulative_bonus_rewards,
            cumulative_real_time_rewards,
            cumulative_total_rewards,
            auto_subscribe,
        }
    }
}

