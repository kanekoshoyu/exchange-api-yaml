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
pub struct SapiV1ConvertTradeFlowGet200ResponseListInner {
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "fromAsset")]
    pub from_asset: String,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "toAsset")]
    pub to_asset: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    /// price ratio
    #[serde(rename = "ratio")]
    pub ratio: String,
    /// inverse price
    #[serde(rename = "inverseRatio")]
    pub inverse_ratio: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
}

impl SapiV1ConvertTradeFlowGet200ResponseListInner {
    pub fn new(quote_id: String, order_id: i64, order_status: String, from_asset: String, from_amount: String, to_asset: String, to_amount: String, ratio: String, inverse_ratio: String, create_time: i64) -> SapiV1ConvertTradeFlowGet200ResponseListInner {
        SapiV1ConvertTradeFlowGet200ResponseListInner {
            quote_id,
            order_id,
            order_status,
            from_asset,
            from_amount,
            to_asset,
            to_amount,
            ratio,
            inverse_ratio,
            create_time,
        }
    }
}

