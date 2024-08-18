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
pub struct SubAccountUsdtFuturesDetailsFutureAccountResp {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "assets")]
    pub assets: Vec<models::SapiV1SubAccountFuturesAccountGet200ResponseAssetsInner>,
    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,
    #[serde(rename = "canTrade")]
    pub can_trade: bool,
    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,
    #[serde(rename = "feeTier")]
    pub fee_tier: i64,
    #[serde(rename = "maxWithdrawAmount")]
    pub max_withdraw_amount: String,
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: String,
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    #[serde(rename = "totalOpenOrderInitialMargin")]
    pub total_open_order_initial_margin: String,
    #[serde(rename = "totalPositionInitialMargin")]
    pub total_position_initial_margin: String,
    #[serde(rename = "totalUnrealizedProfit")]
    pub total_unrealized_profit: String,
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl SubAccountUsdtFuturesDetailsFutureAccountResp {
    pub fn new(email: String, assets: Vec<models::SapiV1SubAccountFuturesAccountGet200ResponseAssetsInner>, can_deposit: bool, can_trade: bool, can_withdraw: bool, fee_tier: i64, max_withdraw_amount: String, total_initial_margin: String, total_maintenance_margin: String, total_margin_balance: String, total_open_order_initial_margin: String, total_position_initial_margin: String, total_unrealized_profit: String, total_wallet_balance: String, update_time: i64) -> SubAccountUsdtFuturesDetailsFutureAccountResp {
        SubAccountUsdtFuturesDetailsFutureAccountResp {
            email,
            assets,
            can_deposit,
            can_trade,
            can_withdraw,
            fee_tier,
            max_withdraw_amount,
            total_initial_margin,
            total_maintenance_margin,
            total_margin_balance,
            total_open_order_initial_margin,
            total_position_initial_margin,
            total_unrealized_profit,
            total_wallet_balance,
            update_time,
        }
    }
}

