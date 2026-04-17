-- 创建回测记录表
CREATE TABLE IF NOT EXISTS backtest_record (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,                      -- 回测名称
    symbols TEXT NOT NULL,                    -- 品种列表 JSON: [{"symbol": "SPY", "weight": 0.6}, ...]
    strategy_type TEXT NOT NULL,              -- 策略类型: fixed_invest, value_averaging
    strategy_params TEXT NOT NULL,            -- 策略参数 JSON
    start_date TEXT NOT NULL,                 -- 回测开始日期
    end_date TEXT NOT NULL,                  -- 回测结束日期
    frequency TEXT NOT NULL,                 -- 频率: weekly, biweekly, monthly
    result_data TEXT,                         -- 回测结果 JSON
    metrics TEXT,                             -- 指标数据 JSON
    status TEXT DEFAULT 'pending',            -- 状态: pending, running, completed, failed
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_backtest_record_status ON backtest_record(status);
CREATE INDEX idx_backtest_record_created_at ON backtest_record(created_at);