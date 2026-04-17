-- 迁移版本: 1
-- 描述: 创建应用日志表
-- 创建时间: 2024-04-16

CREATE TABLE IF NOT EXISTS app_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL CHECK (level IN ('trace', 'debug', 'info', 'warn', 'error')),
    module TEXT NOT NULL CHECK (module IN ('data', 'backtest', 'ui', 'network', 'system')),
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    stack_trace TEXT,
    created_at TEXT DEFAULT (STRFTIME('%Y-%m-%dT%H:%M:%fZ', 'NOW'))
);

-- 创建索引以提升查询性能
CREATE INDEX IF NOT EXISTS idx_app_log_level ON app_log(level);
CREATE INDEX IF NOT EXISTS idx_app_log_module ON app_log(module);
CREATE INDEX IF NOT EXISTS idx_app_log_timestamp ON app_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_app_log_created_at ON app_log(created_at);

-- 创建清理旧日志的触发器（保留最近30天）
-- 注意：实际清理由应用层定时任务执行，这里只提供查询用的视图
CREATE VIEW IF NOT EXISTS app_log_recent AS
SELECT * FROM app_log
WHERE datetime(timestamp) >= datetime('now', '-30 days')
ORDER BY timestamp DESC;
