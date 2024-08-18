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
pub struct SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp {
    #[serde(rename = "totalMarginBalanceOfBTC")]
    pub total_margin_balance_of_btc: String,
    #[serde(rename = "totalUnrealizedProfitOfBTC")]
    pub total_unrealized_profit_of_btc: String,
    #[serde(rename = "totalWalletBalanceOfBTC")]
    pub total_wallet_balance_of_btc: String,
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "subAccountList")]
    pub sub_account_list: Vec<models::SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner>,
}

impl SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp {
    pub fn new(total_margin_balance_of_btc: String, total_unrealized_profit_of_btc: String, total_wallet_balance_of_btc: String, asset: String, sub_account_list: Vec<models::SubAccountCoinFuturesSummaryDeliveryAccountSummaryRespSubAccountListInner>) -> SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp {
        SubAccountCoinFuturesSummaryDeliveryAccountSummaryResp {
            total_margin_balance_of_btc,
            total_unrealized_profit_of_btc,
            total_wallet_balance_of_btc,
            asset,
            sub_account_list,
        }
    }
}

