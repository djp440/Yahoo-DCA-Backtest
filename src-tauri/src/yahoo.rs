//! Yahoo Finance 数据类型定义
//! 与前端 yahoo-finance2 返回格式对齐

use serde::{Deserialize, Serialize};

/// 品种类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SymbolType {
    Stock,
    Etf,
    Crypto,
    Index,
    MutualFund,
    Currency,
    Futures,
}

impl SymbolType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "equity" | "stock" => SymbolType::Stock,
            "etf" => SymbolType::Etf,
            "cryptocurrency" | "crypto" => SymbolType::Crypto,
            "index" => SymbolType::Index,
            "mutualfund" => SymbolType::MutualFund,
            "currency" => SymbolType::Currency,
            "futures" => SymbolType::Futures,
            _ => SymbolType::Stock,
        }
    }

    pub fn to_label(&self) -> &str {
        match self {
            SymbolType::Stock => "美股个股",
            SymbolType::Etf => "美股ETF",
            SymbolType::Crypto => "加密货币",
            SymbolType::Index => "指数",
            SymbolType::MutualFund => "共同基金",
            SymbolType::Currency => "外汇",
            SymbolType::Futures => "期货",
        }
    }
}

/// 日K线价格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBar {
    pub date: String,       // YYYY-MM-DD
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub adj_close: f64,
}

/// 品种搜索结果（yahoo-finance2 search返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YahooSearchResult {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub quote_type: String,
    pub short_name: Option<String>,
}

/// 品种详情（yahoo-finance2 quote返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YahooQuote {
    pub symbol: String,
    pub short_name: String,
    pub long_name: Option<String>,
    pub quote_type: String,
    pub currency: String,
    pub exchange: String,
    pub regular_market_price: Option<f64>,
    pub regular_market_previous_close: Option<f64>,
    pub fifty_two_week_high: Option<f64>,
    pub fifty_two_week_low: Option<f64>,
}

/// 数据下载请求参数
#[derive(Debug, Clone, Deserialize)]
pub struct FetchDataParams {
    pub symbol: String,
    pub start_date: String,  // YYYY-MM-DD
    pub end_date: String,    // YYYY-MM-DD
}

/// 数据下载响应
#[derive(Debug, Clone, Serialize)]
pub struct FetchDataResult {
    pub symbol: String,
    pub count: usize,
    pub first_date: String,
    pub last_date: String,
}
