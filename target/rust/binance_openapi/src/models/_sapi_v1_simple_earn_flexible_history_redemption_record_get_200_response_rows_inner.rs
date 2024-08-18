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
pub struct SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200ResponseRowsInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "redeemId")]
    pub redeem_id: i64,
    /// SPOT, FUNDING
    #[serde(rename = "destAccount")]
    pub dest_account: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200ResponseRowsInner {
    pub fn new(amount: String, asset: String, time: i64, project_id: String, redeem_id: i64, dest_account: String, status: String) -> SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200ResponseRowsInner {
        SapiV1SimpleEarnFlexibleHistoryRedemptionRecordGet200ResponseRowsInner {
            amount,
            asset,
            time,
            project_id,
            redeem_id,
            dest_account,
            status,
        }
    }
}

