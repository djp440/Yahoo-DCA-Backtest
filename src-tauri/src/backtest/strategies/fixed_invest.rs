use serde::{Deserialize, Serialize};

/// 普通定投策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedInvestConfig {
    /// 每次投入金额
    pub invest_amount: f64,
    /// 频率: weekly, biweekly, monthly
    pub frequency: String,
    /// 每周几定投 (0=周日, 1=周一, ..., 6=周六)
    pub weekday: Option<u8>,
    /// 每月几号定投 (1-31)
    pub month_day: Option<u8>,
}

impl Default for FixedInvestConfig {
    fn default() -> Self {
        Self {
            invest_amount: 1000.0,
            frequency: "monthly".to_string(),
            weekday: None,
            month_day: Some(1),
        }
    }
}

/// 定投持仓记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub shares: f64,           // 持有份额
    pub total_cost: f64,       // 总投入成本
    pub avg_cost: f64,         // 平均成本
}

/// 单次定投记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestRecord {
    pub date: String,
    pub price: f64,
    pub amount: f64,
    pub shares: f64,
    pub total_shares: f64,
    pub total_cost: f64,
}

/// 普通定投策略
pub struct FixedInvestStrategy;

impl FixedInvestStrategy {
    /// 计算定投日期列表
    pub fn calculate_invest_dates(
        start_date: &str,
        end_date: &str,
        frequency: &str,
        weekday: Option<u8>,
        month_day: Option<u8>,
    ) -> Vec<String> {
        let mut dates = Vec::new();
        let start = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .expect("Invalid start date");
        let end = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .expect("Invalid end date");

        let mut current = start;
        while current <= end {
            let should_invest = match frequency {
                "weekly" => {
                    if let Some(w) = weekday {
                        current.weekday().num_days_from_sunday() == w
                    } else {
                        current.weekday().num_days_from_sunday() == 1 // 默认周一
                    }
                }
                "biweekly" => {
                    if let Some(w) = weekday {
                        current.weekday().num_days_from_sunday() == w
                    } else {
                        current.weekday().num_days_from_sunday() == 1
                    }
                }
                "monthly" => {
                    if let Some(d) = month_day {
                        current.day() == d || (d > 28 && current.day() >= 28)
                    } else {
                        current.day() == 1
                    }
                }
                _ => false,
            };

            if should_invest {
                dates.push(current.format("%Y-%m-%d").to_string());
            }

            current = match frequency {
                "weekly" | "biweekly" => current + chrono::Duration::days(7),
                "monthly" => {
                    // 简单月计算
                    let month = current.month();
                    let year = current.year();
                    let next_month = if month == 12 { 1 } else { month + 1 };
                    let next_year = if month == 12 { year + 1 } else { year };
                    chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1)
                        .unwrap_or(current + chrono::Duration::days(30))
                }
                _ => current + chrono::Duration::days(1),
            };
        }
        dates
    }

    /// 执行单次定投
    pub fn invest(
        position: &mut Position,
        price: f64,
        amount: f64,
    ) -> InvestRecord {
        let shares = amount / price;
        position.shares += shares;
        position.total_cost += amount;
        position.avg_cost = if position.shares > 0.0 {
            position.total_cost / position.shares
        } else {
            0.0
        };

        InvestRecord {
            date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            price,
            amount,
            shares,
            total_shares: position.shares,
            total_cost: position.total_cost,
        }
    }
}