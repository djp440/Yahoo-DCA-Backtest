pub mod db;
pub mod log;

use crate::db::DbManager;
use crate::log::{LogEntry, LogLevel, LogModule, LogQueryParams, LogService};
use ::log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

/// 应用状态
struct AppState {
    db: Mutex<Option<DbManager>>,
    log_service: Mutex<Option<LogService>>,
}

/// Tauri命令响应格式
#[derive(Serialize)]
struct CommandResult<T> {
    ok: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> CommandResult<T> {
    fn success(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(message),
        }
    }
}

/// 写入日志的参数
#[derive(Deserialize)]
struct WriteLogParams {
    level: String,
    module: String,
    content: String,
    stack_trace: Option<String>,
}

/// 清理日志的参数
#[derive(Deserialize)]
struct ClearLogsParams {
    days: Option<i32>,
}

// ==================== Tauri Commands ====================

#[tauri::command]
async fn log_write(
    params: WriteLogParams,
    state: State<'_, AppState>,
) -> Result<CommandResult<i64>, String> {
    let level = match LogLevel::try_from(params.level.as_str()) {
        Ok(l) => l,
        Err(e) => return Ok(CommandResult::error(e)),
    };

    let module = match params.module.to_lowercase().as_str() {
        "data" => LogModule::Data,
        "backtest" => LogModule::Backtest,
        "ui" => LogModule::Ui,
        "network" => LogModule::Network,
        "system" => LogModule::System,
        _ => LogModule::System,
    };

    let mut log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_mut() {
        match service.write_log(level, module, params.content, params.stack_trace) {
            Ok(id) => Ok(CommandResult::success(id)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("日志服务未初始化".to_string()))
    }
}

#[tauri::command]
async fn log_query(
    params: LogQueryParams,
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<LogEntry>>, String> {
    let log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_ref() {
        match service.query_logs(params) {
            Ok(logs) => Ok(CommandResult::success(logs)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("日志服务未初始化".to_string()))
    }
}

#[tauri::command]
async fn log_clear(
    params: ClearLogsParams,
    state: State<'_, AppState>,
) -> Result<CommandResult<usize>, String> {
    let mut log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_mut() {
        let result = if let Some(days) = params.days {
            service.clear_old_logs(days)
        } else {
            service.clear_all_logs()
        };
        match result {
            Ok(count) => Ok(CommandResult::success(count)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("日志服务未初始化".to_string()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(None),
            log_service: Mutex::new(None),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(LevelFilter::Info)
                        .build(),
                )?;
            }

            // 初始化数据库和日志服务
            let db = DbManager::new(app.handle())?;
            let mut log_service = LogService::new(db);

            // 运行迁移
            log_service.db.run_migrations()?;

            // 保存到状态
            let app_state = app.state::<AppState>();
            *app_state.log_service.lock().unwrap() = Some(log_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![log_write, log_query, log_clear])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
