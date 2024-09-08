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
pub struct SapiV1ManagedSubaccountAssetGet200ResponseInner {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "totalBalance")]
    pub total_balance: String,
    #[serde(rename = "availableBalance")]
    pub available_balance: String,
    #[serde(rename = "inOrder")]
    pub in_order: String,
    #[serde(rename = "btcValue")]
    pub btc_value: String,
}

impl SapiV1ManagedSubaccountAssetGet200ResponseInner {
    pub fn new(coin: String, name: String, total_balance: String, available_balance: String, in_order: String, btc_value: String) -> SapiV1ManagedSubaccountAssetGet200ResponseInner {
        SapiV1ManagedSubaccountAssetGet200ResponseInner {
            coin,
            name,
            total_balance,
            available_balance,
            in_order,
            btc_value,
        }
    }
}

