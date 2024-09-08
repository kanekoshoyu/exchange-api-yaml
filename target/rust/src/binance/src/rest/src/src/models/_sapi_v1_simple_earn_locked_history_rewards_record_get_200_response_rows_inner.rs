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
pub struct SapiV1SimpleEarnLockedHistoryRewardsRecordGet200ResponseRowsInner {
    #[serde(rename = "positionId")]
    pub position_id: String,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "lockPeriod")]
    pub lock_period: String,
    #[serde(rename = "amount")]
    pub amount: String,
}

impl SapiV1SimpleEarnLockedHistoryRewardsRecordGet200ResponseRowsInner {
    pub fn new(position_id: String, time: i64, asset: String, lock_period: String, amount: String) -> SapiV1SimpleEarnLockedHistoryRewardsRecordGet200ResponseRowsInner {
        SapiV1SimpleEarnLockedHistoryRewardsRecordGet200ResponseRowsInner {
            position_id,
            time,
            asset,
            lock_period,
            amount,
        }
    }
}

