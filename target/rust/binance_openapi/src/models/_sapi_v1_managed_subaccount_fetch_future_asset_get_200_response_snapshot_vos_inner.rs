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
pub struct SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "data")]
    pub data: Box<models::SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerData>,
}

impl SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner {
    pub fn new(r#type: String, update_time: i64, data: models::SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerData) -> SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner {
        SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInner {
            r#type,
            update_time,
            data: Box::new(data),
        }
    }
}

