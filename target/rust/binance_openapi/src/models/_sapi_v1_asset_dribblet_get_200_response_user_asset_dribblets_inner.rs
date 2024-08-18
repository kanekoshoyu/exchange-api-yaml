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
pub struct SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInner {
    #[serde(rename = "operateTime")]
    pub operate_time: i64,
    /// Total transfered BNB amount for this exchange.
    #[serde(rename = "totalTransferedAmount")]
    pub total_transfered_amount: String,
    /// Total service charge amount for this exchange.
    #[serde(rename = "totalServiceChargeAmount")]
    pub total_service_charge_amount: String,
    #[serde(rename = "transId")]
    pub trans_id: i64,
    #[serde(rename = "userAssetDribbletDetails")]
    pub user_asset_dribblet_details: Vec<models::SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInnerUserAssetDribbletDetailsInner>,
}

impl SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInner {
    pub fn new(operate_time: i64, total_transfered_amount: String, total_service_charge_amount: String, trans_id: i64, user_asset_dribblet_details: Vec<models::SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInnerUserAssetDribbletDetailsInner>) -> SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInner {
        SapiV1AssetDribbletGet200ResponseUserAssetDribbletsInner {
            operate_time,
            total_transfered_amount,
            total_service_charge_amount,
            trans_id,
            user_asset_dribblet_details,
        }
    }
}

