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

/// ApiV3AccountCommissionGet200ResponseStandardCommission : Standard commission rates on trades from the order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV3AccountCommissionGet200ResponseStandardCommission {
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "taker")]
    pub taker: String,
    #[serde(rename = "buyer")]
    pub buyer: String,
    #[serde(rename = "seller")]
    pub seller: String,
}

impl ApiV3AccountCommissionGet200ResponseStandardCommission {
    /// Standard commission rates on trades from the order.
    pub fn new(maker: String, taker: String, buyer: String, seller: String) -> ApiV3AccountCommissionGet200ResponseStandardCommission {
        ApiV3AccountCommissionGet200ResponseStandardCommission {
            maker,
            taker,
            buyer,
            seller,
        }
    }
}

