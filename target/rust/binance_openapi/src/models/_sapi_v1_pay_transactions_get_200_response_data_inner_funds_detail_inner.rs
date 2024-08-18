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
pub struct SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "amount")]
    pub amount: String,
}

impl SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner {
    pub fn new(currency: String, amount: String) -> SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner {
        SapiV1PayTransactionsGet200ResponseDataInnerFundsDetailInner {
            currency,
            amount,
        }
    }
}

