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
pub struct SapiV1AssetAssetDividendGet200ResponseRowsInner {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "divTime")]
    pub div_time: i64,
    #[serde(rename = "enInfo")]
    pub en_info: String,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
}

impl SapiV1AssetAssetDividendGet200ResponseRowsInner {
    pub fn new(id: i64, amount: String, asset: String, div_time: i64, en_info: String, tran_id: i64) -> SapiV1AssetAssetDividendGet200ResponseRowsInner {
        SapiV1AssetAssetDividendGet200ResponseRowsInner {
            id,
            amount,
            asset,
            div_time,
            en_info,
            tran_id,
        }
    }
}

