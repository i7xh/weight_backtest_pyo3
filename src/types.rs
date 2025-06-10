use chrono::NaiveDateTime;
use polars::prelude::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BacktestInput {
    pub df: DataFrame,
    pub digits: u32,
    pub weight_type: String,
    pub fee_rate: f64,
    pub yearly_days: u32,
    pub n_jobs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyResult {
    pub date: String,
    pub symbol: String,
    pub edge: f64,
    pub return_: f64,
    pub cost: f64,
    pub n1b: f64,
    pub turnover: f64,
    pub long_edge: f64,
    pub long_cost: f64,
    pub long_return: f64,
    pub long_turnover: f64,
    pub short_edge: f64,
    pub short_cost: f64,
    pub short_return: f64,
    pub short_turnover: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePair {
    pub symbol: String,
    pub direction: String,
    pub open_time: String,
    pub close_time: String,
    pub open_price: f64,
    pub close_price: f64,
    pub bars_held: i64,
    pub event_sequence: String,
    pub days_held: i64,
    pub pct_return: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResult {
    pub daily: Vec<DailyResult>,
    pub pairs: Vec<TradePair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub symbol_results: HashMap<String, SymbolResult>,
    pub daily_returns: Vec<DailyReturn>,
    pub stats: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReturn {
    pub date: String,
    pub symbol: String,
    pub return_: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaResult {
    pub date: String,
    pub strategy: f64,
    pub benchmark: f64,
    pub alpha: f64,
}