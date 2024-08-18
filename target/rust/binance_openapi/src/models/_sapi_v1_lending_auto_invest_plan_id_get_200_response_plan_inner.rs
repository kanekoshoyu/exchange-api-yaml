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
pub struct SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInner {
    #[serde(rename = "planId")]
    pub plan_id: i32,
    #[serde(rename = "planType")]
    pub plan_type: String,
    #[serde(rename = "editAllowed")]
    pub edit_allowed: String,
    #[serde(rename = "flexibleAllowedToUse")]
    pub flexible_allowed_to_use: String,
    #[serde(rename = "creationDateTime")]
    pub creation_date_time: i64,
    #[serde(rename = "firstExecutionDateTime")]
    pub first_execution_date_time: i64,
    #[serde(rename = "nextExecutionDateTime")]
    pub next_execution_date_time: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "targetAsset")]
    pub target_asset: String,
    #[serde(rename = "sourceAsset")]
    pub source_asset: String,
    #[serde(rename = "totalInvestedInUSD")]
    pub total_invested_in_usd: String,
    #[serde(rename = "planValueInUSD")]
    pub plan_value_in_usd: String,
    #[serde(rename = "pnlInUSD")]
    pub pnl_in_usd: String,
    #[serde(rename = "roi")]
    pub roi: String,
    #[serde(rename = "details")]
    pub details: Vec<models::SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner>,
}

impl SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInner {
    pub fn new(plan_id: i32, plan_type: String, edit_allowed: String, flexible_allowed_to_use: String, creation_date_time: i64, first_execution_date_time: i64, next_execution_date_time: i64, status: String, target_asset: String, source_asset: String, total_invested_in_usd: String, plan_value_in_usd: String, pnl_in_usd: String, roi: String, details: Vec<models::SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInnerDetailsInner>) -> SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInner {
        SapiV1LendingAutoInvestPlanIdGet200ResponsePlanInner {
            plan_id,
            plan_type,
            edit_allowed,
            flexible_allowed_to_use,
            creation_date_time,
            first_execution_date_time,
            next_execution_date_time,
            status,
            target_asset,
            source_asset,
            total_invested_in_usd,
            plan_value_in_usd,
            pnl_in_usd,
            roi,
            details,
        }
    }
}

