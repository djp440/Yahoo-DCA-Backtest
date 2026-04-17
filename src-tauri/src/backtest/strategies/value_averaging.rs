use serde::{Deserialize, Serialize};

/// 价值平均策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueAveragingConfig {
    /// 目标市值增长率 (每月)
    pub target_growth_rate: f64,  // 如 0.02 表示每月增长2%
    /// 初始市值
    pub initial_value: f64,
    /// 最大单次投入
    pub max_investment: f64,
    /// 最小单次投入
    pub min_investment: f64,
    /// 频率: monthly
    pub frequency: String,
    /// 每月几号定投
    pub month_day: u8,
}

impl Default for ValueAveragingConfig {
    fn default() -> Self {
        Self {
            target_growth_rate: 0.02,
            initial_value: 1000.0,
            max_investment: 5000.0,
            min_investment: 0.0,
            frequency: "monthly".to_string(),
            month_day: 1,
        }
    }
}

/// 价值平均策略持仓
#[derive(Debug, Clone)]
pub struct ValuePosition {
    pub shares: f64,
    pub total_cost: f64,
    pub target_value: f64,
}

impl ValuePosition {
    pub fn new(initial_value: f64, initial_price: f64) -> Self {
        let shares = initial_value / initial_price;
        Self {
            shares,
            total_cost: initial_value,
            target_value: initial_value,
        }
    }

    /// 计算应投入金额
    pub fn calculate_investment(&self, current_price: f64) -> f64 {
        let current_value = self.shares * current_price;
        let diff = self.target_value - current_value;
        diff
    }

    /// 更新目标市值
    pub fn update_target(&mut self, growth_rate: f64) {
        self.target_value *= (1.0 + growth_rate);
    }
}