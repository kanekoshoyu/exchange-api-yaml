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
pub struct SapiV1CapitalWithdrawAddressListGet200ResponseInner {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addressTag")]
    pub address_tag: String,
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "network")]
    pub network: String,
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(rename = "originType")]
    pub origin_type: String,
    #[serde(rename = "whiteStatus")]
    pub white_status: bool,
}

impl SapiV1CapitalWithdrawAddressListGet200ResponseInner {
    pub fn new(address: String, address_tag: String, coin: String, name: String, network: String, origin: String, origin_type: String, white_status: bool) -> SapiV1CapitalWithdrawAddressListGet200ResponseInner {
        SapiV1CapitalWithdrawAddressListGet200ResponseInner {
            address,
            address_tag,
            coin,
            name,
            network,
            origin,
            origin_type,
            white_status,
        }
    }
}

