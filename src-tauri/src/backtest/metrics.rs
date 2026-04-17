use serde::{Deserialize, Serialize};

/// 回测每日数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyData {
    pub date: String,
    pub price: f64,
    pub value: f64,        // 当日组合市值
    pub cost: f64,         // 累计投入成本
    pub drawdown: f64,     // 当日回撤
}

/// 收益指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnMetrics {
    pub total_return: f64,           // 总收益率
    pub annualized_return: f64,     // 年化收益率
    pub final_value: f64,            // 复利终值
    pub total_invested: f64,        // 总投入本金
    pub profit: f64,                // 绝对收益
}

/// 风险指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub max_drawdown: f64,          // 最大回撤
    pub max_drawdown_duration: u32, // 最大回撤持续天数
    pub volatility: f64,             // 波动率(标准差)
    pub downside_deviation: f64,    // 下行波动率
    pub var_95: f64,                // VaR 95%
    pub cvar_95: f64,               // CVaR 95%
}

/// 风险调整收益指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAdjustedMetrics {
    pub sharpe_ratio: f64,          // 夏普比率
    pub sortino_ratio: f64,          // 索提诺比率
    pub calmar_ratio: f64,          // 卡玛比率
    pub omega_ratio: f64,            // 欧米伽比率
}

/// 定投相关指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCAMetrics {
    pub total_invested: f64,       // 投入本金总计
    pub market_value: f64,          // 持仓市值
    pub invest_count: u32,          // 投入次数
    pub avg_cost: f64,              // 平均持仓成本
    pub dca_return: f64,            // 定投收益率
    pub lump_sum_return: f64,       // 一次性投入收益率
    pub breakeven_point: f64,       // 盈亏平衡点
}

/// 完整回测指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestMetrics {
    pub returns: ReturnMetrics,
    pub risk: RiskMetrics,
    pub risk_adjusted: RiskAdjustedMetrics,
    pub dca: DCAMetrics,
}

pub struct MetricsCalculator;

impl MetricsCalculator {
    /// 计算总收益率
    pub fn calculate_total_return(final_value: f64, total_invested: f64) -> f64 {
        if total_invested <= 0.0 {
            return 0.0;
        }
        (final_value - total_invested) / total_invested
    }

    /// 计算年化收益率
    pub fn calculate_annualized_return(total_return: f64, days: u32) -> f64 {
        if days == 0 {
            return 0.0;
        }
        let years = days as f64 / 365.25;
        if years <= 0.0 {
            return 0.0;
        }
        // (1 + r)^(1/y) - 1
        (1.0 + total_return).powf(1.0 / years) - 1.0
    }

    /// 计算最大回撤
    pub fn calculate_max_drawdown(values: &[f64]) -> (f64, usize, usize) {
        if values.is_empty() {
            return (0.0, 0, 0);
        }

        let mut max_dd = 0.0;
        let mut peak = values[0];
        let mut peak_idx = 0;
        let mut max_dd_start = 0;
        let mut max_dd_end = 0;

        for (i, &value) in values.iter().enumerate() {
            if value > peak {
                peak = value;
                peak_idx = i;
            }

            let dd = (peak - value) / peak;
            if dd > max_dd {
                max_dd = dd;
                max_dd_start = peak_idx;
                max_dd_end = i;
            }
        }

        (max_dd, max_dd_start, max_dd_end)
    }

    /// 计算波动率 (标准差)
    pub fn calculate_volatility(returns: &[f64]) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;

        variance.sqrt()
    }

    /// 计算夏普比率
    pub fn calculate_sharpe_ratio(annualized_return: f64, volatility: f64, risk_free_rate: f64) -> f64 {
        if volatility <= 0.0 {
            return 0.0;
        }
        (annualized_return - risk_free_rate) / volatility
    }

    /// 计算索提诺比率
    pub fn calculate_sortino_ratio(annualized_return: f64, downside_dev: f64, risk_free_rate: f64) -> f64 {
        if downside_dev <= 0.0 {
            return 0.0;
        }
        (annualized_return - risk_free_rate) / downside_dev
    }
}