pub mod db;
pub mod log;
pub mod data;
pub mod yahoo;

use crate::db::DbManager;
use crate::log::{LogEntry, LogLevel, LogModule, LogQueryParams, LogService};
use crate::data::{DataService, PriceQueryParams, SymbolListParams, SymbolMeta};
use ::log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, State};

/// 应用状态
struct AppState {
    db: Mutex<Option<DbManager>>,
    log_service: Mutex<Option<LogService>>,
    data_service: Mutex<Option<DataService>>,
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

#[tauri::command]
async fn data_store(
    symbol: String,
    quote: yahoo::YahooQuote,
    prices: Vec<yahoo::PriceBar>,
    state: State<'_, AppState>,
) -> Result<CommandResult<yahoo::FetchDataResult>, String> {
    let mut data_service = state.data_service.lock().unwrap();
    if let Some(service) = data_service.as_mut() {
        if let Err(e) = service.upsert_symbol(&quote) {
            return Ok(CommandResult::error(format!("保存品种信息失败: {}", e)));
        }

        match service.save_price_data(&symbol, &prices) {
            Ok(count) => {
                let first_date = prices.first().map(|p| p.date.clone()).unwrap_or_default();
                let last_date = prices.last().map(|p| p.date.clone()).unwrap_or_default();
                Ok(CommandResult::success(yahoo::FetchDataResult {
                    symbol,
                    count,
                    first_date,
                    last_date,
                }))
            }
            Err(e) => Ok(CommandResult::error(format!("保存价格数据失败: {}", e))),
        }
    } else {
        Ok(CommandResult::error("数据服务未初始化".to_string()))
    }
}

#[tauri::command]
async fn data_query(
    params: PriceQueryParams,
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<yahoo::PriceBar>>, String> {
    let data_service = state.data_service.lock().unwrap();
    if let Some(service) = data_service.as_ref() {
        match service.query_prices(params) {
            Ok(prices) => Ok(CommandResult::success(prices)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("数据服务未初始化".to_string()))
    }
}

#[tauri::command]
async fn data_list_symbols(
    params: SymbolListParams,
    state: State<'_, AppState>,
) -> Result<CommandResult<Vec<SymbolMeta>>, String> {
    let data_service = state.data_service.lock().unwrap();
    if let Some(service) = data_service.as_ref() {
        match service.list_symbols(params) {
            Ok(symbols) => Ok(CommandResult::success(symbols)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("数据服务未初始化".to_string()))
    }
}

#[tauri::command]
async fn data_delete_symbol(
    symbol: String,
    state: State<'_, AppState>,
) -> Result<CommandResult<usize>, String> {
    let mut data_service = state.data_service.lock().unwrap();
    if let Some(service) = data_service.as_mut() {
        match service.delete_symbol(&symbol) {
            Ok(count) => Ok(CommandResult::success(count)),
            Err(e) => Ok(CommandResult::error(e.to_string())),
        }
    } else {
        Ok(CommandResult::error("数据服务未初始化".to_string()))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(None),
            log_service: Mutex::new(None),
            data_service: Mutex::new(None),
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(LevelFilter::Info)
                        .build(),
                )?;
            }

            let mut db = DbManager::new(app.handle())?;
            db.run_migrations()?;

            let log_service = LogService::new(DbManager::new(app.handle())?);
            let data_service = DataService::new(DbManager::new(app.handle())?);

            let app_state = app.state::<AppState>();
            *app_state.log_service.lock().unwrap() = Some(log_service);
            *app_state.data_service.lock().unwrap() = Some(data_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            log_write, log_query, log_clear,
            data_store, data_query, data_list_symbols, data_delete_symbol,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
