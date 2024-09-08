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
pub struct SapiV1SubAccountUniversalTransferGet200ResponseInner {
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "fromEmail")]
    pub from_email: String,
    #[serde(rename = "toEmail")]
    pub to_email: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "fromAccountType")]
    pub from_account_type: String,
    #[serde(rename = "toAccountType")]
    pub to_account_type: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createTimeStamp")]
    pub create_time_stamp: i64,
    #[serde(rename = "clientTranId")]
    pub client_tran_id: String,
}

impl SapiV1SubAccountUniversalTransferGet200ResponseInner {
    pub fn new(tran_id: i64, from_email: String, to_email: String, asset: String, amount: String, from_account_type: String, to_account_type: String, status: String, create_time_stamp: i64, client_tran_id: String) -> SapiV1SubAccountUniversalTransferGet200ResponseInner {
        SapiV1SubAccountUniversalTransferGet200ResponseInner {
            tran_id,
            from_email,
            to_email,
            asset,
            amount,
            from_account_type,
            to_account_type,
            status,
            create_time_stamp,
            client_tran_id,
        }
    }
}

