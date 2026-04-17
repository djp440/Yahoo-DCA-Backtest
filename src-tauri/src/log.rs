//! 日志模块 - 应用日志的写入、查询和清理

use crate::db::{DbManager, DbResult};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};

/// 日志级别
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Trace => "trace".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        }
    }
}

impl TryFrom<&str> for LogLevel {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("无效的日志级别: {}", s)),
        }
    }
}

/// 日志模块
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogModule {
    Data,
    Backtest,
    Ui,
    Network,
    System,
}

impl ToString for LogModule {
    fn to_string(&self) -> String {
        match self {
            LogModule::Data => "data".to_string(),
            LogModule::Backtest => "backtest".to_string(),
            LogModule::Ui => "ui".to_string(),
            LogModule::Network => "network".to_string(),
            LogModule::System => "system".to_string(),
        }
    }
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Option<i64>,
    pub level: String,
    pub module: String,
    pub content: String,
    pub timestamp: String,
    pub stack_trace: Option<String>,
}

/// 日志查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogQueryParams {
    pub level: Option<String>,
    pub module: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// 日志服务
pub struct LogService {
    pub(crate) db: DbManager,
}

impl LogService {
    /// 创建新的日志服务
    pub fn new(db: DbManager) -> Self {
        Self { db }
    }

    /// 写入日志
    pub fn write_log(
        &mut self,
        level: LogLevel,
        module: LogModule,
        content: String,
        stack_trace: Option<String>,
    ) -> DbResult<i64> {
        let timestamp = Utc::now().to_rfc3339();

        self.db.conn().execute(
            "INSERT INTO app_log (level, module, content, timestamp, stack_trace)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                level.to_string(),
                module.to_string(),
                content,
                timestamp,
                stack_trace,
            ],
        )?;

        Ok(self.db.conn().last_insert_rowid())
    }

    /// 查询日志
    pub fn query_logs(&self, params: LogQueryParams) -> DbResult<Vec<LogEntry>> {
        let mut sql = String::from(
            "SELECT id, level, module, content, timestamp, stack_trace
             FROM app_log WHERE 1=1",
        );
        let mut sql_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        let mut param_index = 1;

        if let Some(level) = &params.level {
            sql.push_str(&format!(" AND level = ?{}", param_index));
            sql_params.push(Box::new(level.clone()));
            param_index += 1;
        }

        if let Some(module) = &params.module {
            sql.push_str(&format!(" AND module = ?{}", param_index));
            sql_params.push(Box::new(module.clone()));
            param_index += 1;
        }

        if let Some(start_time) = &params.start_time {
            sql.push_str(&format!(" AND timestamp >= ?{}", param_index));
            sql_params.push(Box::new(start_time.clone()));
            param_index += 1;
        }

        if let Some(end_time) = &params.end_time {
            sql.push_str(&format!(" AND timestamp <= ?{}", param_index));
            sql_params.push(Box::new(end_time.clone()));
            param_index += 1;
        }

        sql.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = params.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        } else {
            sql.push_str(" LIMIT 100");
        }

        if let Some(offset) = params.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db.conn().prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = sql_params.iter().map(|p| p.as_ref()).collect();

        let logs = stmt.query_map(&param_refs[..], |row| {
            Ok(LogEntry {
                id: Some(row.get(0)?),
                level: row.get(1)?,
                module: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
                stack_trace: row.get(5)?,
            })
        })?;

        logs.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// 清理旧日志（保留最近days天）
    pub fn clear_old_logs(&mut self, days: i32) -> DbResult<usize> {
        let deleted = self.db.conn().execute(
            "DELETE FROM app_log WHERE datetime(timestamp) < datetime('now', ?1)",
            params![format!("-{} days", days)],
        )?;
        Ok(deleted)
    }

    /// 清空所有日志
    pub fn clear_all_logs(&mut self) -> DbResult<usize> {
        let deleted = self.db.conn().execute("DELETE FROM app_log", [])?;
        Ok(deleted)
    }
}
