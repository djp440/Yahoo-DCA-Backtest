-- 品种元数据表
CREATE TABLE IF NOT EXISTS symbol_meta (
    symbol TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    exchange TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    first_date TEXT,
    last_date TEXT,
    total_records INTEGER NOT NULL DEFAULT 0,
    last_update TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
