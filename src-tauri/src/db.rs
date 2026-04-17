//! 数据库模块 - SQLite连接与基础操作

use rusqlite::{params, Connection, OptionalExtension};
use tauri::Manager;

/// 数据库错误类型
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("数据库连接错误: {0}")]
    Connection(#[from] rusqlite::Error),
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("初始化错误: {0}")]
    Init(String),
}

pub type DbResult<T> = Result<T, DbError>;

/// 数据库管理器
pub struct DbManager {
    conn: Connection,
}

impl DbManager {
    /// 创建新的数据库管理器
    pub fn new(app_handle: &tauri::AppHandle) -> DbResult<Self> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| DbError::Init(format!("无法获取应用数据目录: {}", e)))?;

        std::fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("dca_backtest.db");
        let conn = Connection::open(&db_path)?;

        // 启用外键约束
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let manager = Self { conn };
        manager.init_tables()?;

        Ok(manager)
    }

    /// 获取数据库连接引用
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// 获取数据库连接的可变引用
    pub fn conn_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }

    /// 初始化数据库表
    fn init_tables(&self) -> DbResult<()> {
        // 创建数据库版本表
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS db_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );",
        )?;

        // 检查是否需要迁移
        let current_version: Option<i32> = self
            .conn
            .query_row("SELECT version FROM db_version ORDER BY version DESC LIMIT 1", [], |row| {
                row.get(0)
            })
            .optional()?;

        if current_version.is_none() {
            // 初始版本
            self.conn.execute(
                "INSERT INTO db_version (version, applied_at) VALUES (?1, ?2)",
                params![0, chrono::Utc::now().to_rfc3339()],
            )?;
        }

        Ok(())
    }

    /// 执行迁移脚本
    pub fn apply_migration(&mut self, version: i32, sql: &str) -> DbResult<()> {
        let tx = self.conn.unchecked_transaction()?;

        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO db_version (version, applied_at) VALUES (?1, ?2)",
            params![version, chrono::Utc::now().to_rfc3339()],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// 获取当前数据库版本
    pub fn get_current_version(&self) -> DbResult<i32> {
        Ok(self
            .conn
            .query_row("SELECT COALESCE(MAX(version), 0) FROM db_version", [], |row| {
                row.get(0)
            })?)
    }

    /// 运行所有待应用的迁移
    pub fn run_migrations(&mut self) -> DbResult<()> {
        let current_version = self.get_current_version()?;

        // 迁移1: 创建日志表
        if current_version < 1 {
            let migration_sql = include_str!("../../migrations/20240416_create_app_log_table.sql");
            self.apply_migration(1, migration_sql)?;
        }

        // 迁移2: 创建价格数据表
        if current_version < 2 {
            let migration_sql = include_str!("../../migrations/20240417_create_price_data_table.sql");
            self.apply_migration(2, migration_sql)?;
        }

        // 迁移3: 创建品种元数据表
        if current_version < 3 {
            let migration_sql = include_str!("../../migrations/20240417_create_symbol_meta_table.sql");
            self.apply_migration(3, migration_sql)?;
        }

        Ok(())
    }
}
