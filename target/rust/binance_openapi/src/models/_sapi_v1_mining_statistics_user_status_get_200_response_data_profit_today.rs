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
pub struct SapiV1MiningStatisticsUserStatusGet200ResponseDataProfitToday {
    #[serde(rename = "BTC")]
    pub btc: String,
    #[serde(rename = "BSV")]
    pub bsv: String,
    #[serde(rename = "BCH")]
    pub bch: String,
}

impl SapiV1MiningStatisticsUserStatusGet200ResponseDataProfitToday {
    pub fn new(btc: String, bsv: String, bch: String) -> SapiV1MiningStatisticsUserStatusGet200ResponseDataProfitToday {
        SapiV1MiningStatisticsUserStatusGet200ResponseDataProfitToday {
            btc,
            bsv,
            bch,
        }
    }
}

