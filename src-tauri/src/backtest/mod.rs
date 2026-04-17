/// 回测模块入口
pub mod strategies;
pub mod metrics;
pub mod engine;

pub use strategies::{FixedInvestStrategy, ValueAveragingStrategy};
pub use metrics::{BacktestMetrics, ReturnMetrics, RiskMetrics, DCAMetrics};
pub use engine::{BacktestEngine, BacktestRequest, BacktestResponse};
