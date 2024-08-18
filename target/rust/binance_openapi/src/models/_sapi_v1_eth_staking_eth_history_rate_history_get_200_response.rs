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
pub struct SapiV1EthStakingEthHistoryRateHistoryGet200Response {
    #[serde(rename = "rows")]
    pub rows: Vec<models::SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner>,
    #[serde(rename = "total")]
    pub total: i64,
}

impl SapiV1EthStakingEthHistoryRateHistoryGet200Response {
    pub fn new(rows: Vec<models::SapiV1EthStakingEthHistoryRateHistoryGet200ResponseRowsInner>, total: i64) -> SapiV1EthStakingEthHistoryRateHistoryGet200Response {
        SapiV1EthStakingEthHistoryRateHistoryGet200Response {
            rows,
            total,
        }
    }
}

