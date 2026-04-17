-- 品种日K线价格数据表
CREATE TABLE IF NOT EXISTS price_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    date TEXT NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume INTEGER NOT NULL DEFAULT 0,
    adj_close REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(symbol, date)
);

-- 查询优化索引
CREATE INDEX IF NOT EXISTS idx_price_data_symbol ON price_data(symbol);
CREATE INDEX IF NOT EXISTS idx_price_data_date ON price_data(date);
CREATE INDEX IF NOT EXISTS idx_price_data_symbol_date ON price_data(symbol, date);
