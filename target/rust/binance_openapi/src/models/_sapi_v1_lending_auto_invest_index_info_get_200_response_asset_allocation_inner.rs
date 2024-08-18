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
pub struct SapiV1LendingAutoInvestIndexInfoGet200ResponseAssetAllocationInner {
    #[serde(rename = "targetAsset")]
    pub target_asset: String,
    #[serde(rename = "allocation")]
    pub allocation: String,
}

impl SapiV1LendingAutoInvestIndexInfoGet200ResponseAssetAllocationInner {
    pub fn new(target_asset: String, allocation: String) -> SapiV1LendingAutoInvestIndexInfoGet200ResponseAssetAllocationInner {
        SapiV1LendingAutoInvestIndexInfoGet200ResponseAssetAllocationInner {
            target_asset,
            allocation,
        }
    }
}

