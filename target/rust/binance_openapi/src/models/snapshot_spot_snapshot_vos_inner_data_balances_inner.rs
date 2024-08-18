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
pub struct SnapshotSpotSnapshotVosInnerDataBalancesInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "free")]
    pub free: String,
    #[serde(rename = "locked")]
    pub locked: String,
}

impl SnapshotSpotSnapshotVosInnerDataBalancesInner {
    pub fn new(asset: String, free: String, locked: String) -> SnapshotSpotSnapshotVosInnerDataBalancesInner {
        SnapshotSpotSnapshotVosInnerDataBalancesInner {
            asset,
            free,
            locked,
        }
    }
}

