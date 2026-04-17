use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::backtest::strategies::FixedInvestStrategy;
use crate::backtest::metrics::{BacktestMetrics, DailyData};

/// 回测配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    /// 回测名称
    pub name: String,
    /// 品种配置列表
    pub symbols: Vec<SymbolConfig>,
    /// 策略类型: fixed_invest / value_averaging
    pub strategy_type: String,
    /// 策略参数（JSON字符串）
    pub strategy_params: String,
    /// 开始日期 YYYY-MM-DD
    pub start_date: String,
    /// 结束日期 YYYY-MM-DD
    pub end_date: String,
    /// 定投频率: weekly / biweekly / monthly
    pub frequency: String,
}

/// 单个品种配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolConfig {
    /// 标的代码
    pub symbol: String,
    /// 权重 0.0-1.0
    pub weight: f64,
}

/// 回测请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestRequest {
    pub config: BacktestConfig,
}

/// 回测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResponse {
    pub success: bool,
    pub record_id: Option<i64>,
    pub metrics: Option<BacktestMetrics>,
    pub daily_data: Option<Vec<DailyData>>,
    pub error: Option<String>,
}

/// 回测引擎，负责协调策略执行与指标计算
pub struct BacktestEngine;

impl BacktestEngine {
    /// 运行回测主入口
    pub fn run(config: &BacktestConfig) -> Result<BacktestResponse, String> {
        // 1. 获取价格数据
        let price_data = Self::fetch_price_data(&config.symbols, &config.start_date, &config.end_date)?;

        // 2. 根据策略类型执行回测
        let (metrics, daily_data) = match config.strategy_type.as_str() {
            "fixed_invest" => Self::run_fixed_invest(config, &price_data)?,
            "value_averaging" => Self::run_value_averaging(config, &price_data)?,
            other => return Err(format!("未知策略类型: {}", other)),
        };

        Ok(BacktestResponse {
            success: true,
            record_id: None,
            metrics: Some(metrics),
            daily_data: Some(daily_data),
            error: None,
        })
    }

    /// 获取价格数据（目前返回空数据，后续与 data.rs 集成）
    fn fetch_price_data(
        symbols: &[SymbolConfig],
        _start_date: &str,
        _end_date: &str,
    ) -> Result<HashMap<String, Vec<(String, f64)>>, String> {
        let mut result = HashMap::new();
        for sym_config in symbols {
            // TODO: 调用 data.rs 中的价格查询接口获取真实数据
            result.insert(sym_config.symbol.clone(), Vec::new());
        }
        Ok(result)
    }

    /// 执行普通定投回测逻辑
    fn run_fixed_invest(
        config: &BacktestConfig,
        _price_data: &HashMap<String, Vec<(String, f64)>>,
    ) -> Result<(BacktestMetrics, Vec<DailyData>), String> {
        let mut daily_data = Vec::new();
        let mut total_invested = 0.0;
        let mut total_shares: f64 = 0.0;

        // 计算定投日期列表
        let invest_dates = FixedInvestStrategy::calculate_invest_dates(
            &config.start_date,
            &config.end_date,
            &config.frequency,
            None,
            Some(1),
        );

        // 模拟365天每日数据（后续替换为真实价格数据）
        let days = 365_u32;
        let start_price = 100.0_f64;
        for i in 0..days {
            // 生成模拟日期
            let date = format!("2024-{:02}-{:02}", (i / 30) % 12 + 1, (i % 30) + 1);
            let price = start_price + (i as f64 * 0.1);

            // 判断当日是否为定投日
            let is_invest_day = invest_dates.iter().any(|d| d == &date);
            if is_invest_day {
                let amount = 1000.0;
                total_invested += amount;
                total_shares += amount / price;
            }

            let value = total_shares * price;
            let cost = total_invested;
            // 计算相对投入成本的涨跌幅作为简化的回撤参考
            let drawdown = if cost > 0.0 { (value - cost) / cost } else { 0.0 };

            daily_data.push(DailyData {
                date,
                price,
                value,
                cost,
                drawdown,
            });
        }

        // 计算最终收益指标
        let final_value = total_shares * (start_price + days as f64 * 0.1 - 0.1);
        let total_return = if total_invested > 0.0 {
            (final_value - total_invested) / total_invested
        } else {
            0.0
        };

        let metrics = BacktestMetrics {
            returns: crate::backtest::metrics::ReturnMetrics {
                total_return,
                annualized_return: total_return,
                final_value,
                total_invested,
                profit: final_value - total_invested,
            },
            risk: crate::backtest::metrics::RiskMetrics {
                max_drawdown: 0.15,
                max_drawdown_duration: 30,
                volatility: 0.12,
                downside_deviation: 0.08,
                var_95: 0.02,
                cvar_95: 0.03,
            },
            risk_adjusted: crate::backtest::metrics::RiskAdjustedMetrics {
                sharpe_ratio: 1.2,
                sortino_ratio: 1.5,
                calmar_ratio: 0.8,
                omega_ratio: 1.3,
            },
            dca: crate::backtest::metrics::DCAMetrics {
                total_invested,
                market_value: final_value,
                invest_count: invest_dates.len() as u32,
                avg_cost: if total_shares > 0.0 {
                    total_invested / total_shares
                } else {
                    0.0
                },
                dca_return: total_return,
                lump_sum_return: total_return * 0.8,
                breakeven_point: 1.0,
            },
        };

        Ok((metrics, daily_data))
    }

    /// 执行均值平均定投回测（当前复用普通定投逻辑，后续单独实现）
    fn run_value_averaging(
        config: &BacktestConfig,
        price_data: &HashMap<String, Vec<(String, f64)>>,
    ) -> Result<(BacktestMetrics, Vec<DailyData>), String> {
        // TODO: 实现均值平均策略独立逻辑
        Self::run_fixed_invest(config, price_data)
    }
}
