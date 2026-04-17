# 阶段1：项目初始化与基础框架 (Milestone 1) 实施计划

&gt; **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 完成项目基础框架搭建，包括Vite+Vue3配置、Tailwind CSS主题、日志模块、基础UI组件等。

**Architecture:** 严格按照TODO.md中的原子化任务顺序执行，每个任务只修改单个文件，确保验收通过后再进行下一任务。

**Tech Stack:** Tauri 2.0 + Vue 3 + Vite + Tailwind CSS + TypeScript + Rust + SQLite

---

## 任务概览

| ID | 任务描述 | 状态 |
|----|----------|------|
| 1.1 | 初始化Tauri 2.0项目配置 | ✅ 已完成 |
| 1.2 | 配置Vite + Vue 3基础工程 | ⏳ 待开始 |
| 1.3 | 配置Tailwind CSS主题 | ⏳ 待开始 |
| 1.4 | 配置全局样式 | ⏳ 待开始 |
| 1.5 | TypeScript全局类型定义 | ⏳ 待开始 |
| 1.6 | Rust数据库连接配置 | ⏳ 待开始 |
| 1.7 | 日志表数据库迁移 | ⏳ 待开始 |
| 1.8 | 日志模块Rust后端实现 | ⏳ 待开始 |
| 1.9 | 注册日志Tauri命令 | ⏳ 待开始 |
| 1.10 | 日志模块前端SDK封装 | ⏳ 待开始 |
| 1.11 | 顶部导航栏组件实现 | ⏳ 待开始 |
| 1.12 | Tab标签栏组件实现 | ⏳ 待开始 |
| 1.13 | 工作区容器组件实现 | ⏳ 待开始 |
| 1.14 | 全局骨架屏组件实现 | ⏳ 待开始 |
| 1.15 | 全局进度条组件实现 | ⏳ 待开始 |
| 1.16 | Vue路由配置 | ⏳ 待开始 |
| 1.17 | 根布局组件实现 | ⏳ 待开始 |
| 1.18 | 首页/仪表盘基础结构 | ⏳ 待开始 |
| 1.19 | 基础按钮组件实现 | ⏳ 待开始 |
| 1.20 | 基础卡片组件实现 | ⏳ 待开始 |

---

## 前置准备

首先安装必要的依赖：

- [ ] **Step 1: 安装Tailwind CSS及相关依赖**

```bash
npm install -D tailwindcss postcss autoprefixer
npm install vue-router@4 pinia
```

- [ ] **Step 2: 安装Rust数据库依赖**

需要修改 `src-tauri/Cargo.toml` 添加：
- rusqlite (SQLite驱动)
- chrono (时间处理)
- serde (序列化)

---

## 详细任务分解

### Task 1.2: 配置Vite + Vue 3基础工程

**Files:**
- Modify: `vite.config.ts`

**验收标准:** 配置端口、路径别名，Vue3 + TypeScript编译正常

- [ ] **Step 1: 修改 vite.config.ts**

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@views': path.resolve(__dirname, './src/views'),
      '@utils': path.resolve(__dirname, './src/utils'),
      '@services': path.resolve(__dirname, './src/services'),
      '@stores': path.resolve(__dirname, './src/stores'),
      '@types': path.resolve(__dirname, './src/types'),
      '@assets': path.resolve(__dirname, './src/assets'),
    }
  },
  server: {
    port: 5173,
    strictPort: true
  },
  clearScreen: false
})
```

- [ ] **Step 2: 验证配置**

运行: `npm run dev`
Expected: Vite正常启动，无配置错误

---

### Task 1.3: 配置Tailwind CSS主题

**Files:**
- Create: `tailwind.config.js`
- Create: `postcss.config.js`
- Modify: `package.json` (确保依赖已安装)

**验收标准:** 完全按UI_DESIGN.md配置颜色、字体、圆角、阴影、动画token

- [ ] **Step 1: 创建 tailwind.config.js**

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#007AFF',
          light: '#E1F0FF',
          dark: '#0051A8',
        },
        success: '#34C759',
        warning: '#FF9500',
        danger: '#FF3B30',
        up: '#FF3B30',
        down: '#34C759',
        neutral: {
          50: '#FFFFFF',
          100: '#F8F9FA',
          200: '#E9ECEF',
          300: '#ADB5BD',
          400: '#495057',
          500: '#212529',
        }
      },
      fontFamily: {
        sans: ['-apple-system', 'BlinkMacSystemFont', '"SF Pro"', 'Inter', 'sans-serif'],
      },
      fontSize: {
        'display-lg': ['32px', { lineHeight: '1.2', fontWeight: '700' }],
        'heading-1': ['24px', { lineHeight: '1.3', fontWeight: '600' }],
        'heading-2': ['20px', { lineHeight: '1.3', fontWeight: '600' }],
        'heading-3': ['16px', { lineHeight: '1.4', fontWeight: '600' }],
        'body': ['14px', { lineHeight: '1.5', fontWeight: '400' }],
        'caption': ['12px', { lineHeight: '1.5', fontWeight: '400' }],
        'tiny': ['10px', { lineHeight: '1.5', fontWeight: '400' }],
      },
      borderRadius: {
        'card': '12px',
        'modal': '16px',
        'button': '8px',
        'input': '8px',
        'tag': '6px',
      },
      boxShadow: {
        'card': '0 1px 2px rgba(0, 0, 0, 0.05)',
        'default': '0 4px 12px rgba(0, 0, 0, 0.08)',
        'hover': '0 8px 24px rgba(0, 0, 0, 0.12)',
        'modal': '0 12px 48px rgba(0, 0, 0, 0.16)',
      },
      animation: {
        'spring': 'spring 0.3s ease-out',
        'fade-in': 'fadeIn 0.25s ease-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'slide-down': 'slideDown 0.3s ease-out',
        'scale-in': 'scaleIn 0.3s ease-out',
        'pulse-soft': 'pulseSoft 2s ease-in-out infinite',
      },
      keyframes: {
        spring: {
          '0%': { transform: 'scale(0.95)' },
          '100%': { transform: 'scale(1)' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.9)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        pulseSoft: {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.7' },
        },
      },
      transitionTimingFunction: {
        'spring': 'cubic-bezier(0.175, 0.885, 0.32, 1.275)',
      }
    },
  },
  plugins: [],
}
```

- [ ] **Step 2: 创建 postcss.config.js**

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

---

### Task 1.4: 配置全局样式

**Files:**
- Modify: `src/style.css`

**验收标准:** 引入Tailwind基础样式，全局重置样式符合苹果设计风格

- [ ] **Step 1: 修改 src/style.css**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro", "Inter", sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background: #F8F9FA;
  color: #212529;
}

@layer utilities {
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
}
```

---

### Task 1.5: TypeScript全局类型定义

**Files:**
- Create: `src/types/global.d.ts`

**验收标准:** 定义通用类型、Tauri命令返回值类型、日志类型

- [ ] **Step 1: 创建 src/types/global.d.ts**

```typescript
/**
 * 全局TypeScript类型定义
 */

// ==================== 通用API响应类型 ====================
export interface ApiResponse&lt;T = any&gt; {
  success: boolean
  data?: T
  error?: string
  meta?: {
    total?: number
    page?: number
    limit?: number
  }
}

// ==================== 日志相关类型 ====================
export type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error'

export type LogModule = 'data' | 'backtest' | 'ui' | 'network' | 'system'

export interface LogEntry {
  id?: number
  level: LogLevel
  module: LogModule
  content: string
  timestamp: string
  stackTrace?: string
}

export interface LogQueryParams {
  level?: LogLevel
  module?: LogModule
  startTime?: string
  endTime?: string
  limit?: number
  offset?: number
}

// ==================== Tauri命令返回值类型 ====================
export interface TauriCommandResult&lt;T = any&gt; {
  ok: boolean
  data?: T
  error?: string
}

// ==================== 通用工具类型 ====================
export type Nullable&lt;T&gt; = T | null

export type Optional&lt;T&gt; = T | undefined

export type DeepReadonly&lt;T&gt; = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly&lt;T[P]&gt; : T[P]
}

export interface DateRange {
  start: string
  end: string
}

export interface SortOption {
  field: string
  order: 'asc' | 'desc'
}

// ==================== Vue相关类型扩展 ====================
declare module 'vue' {
  interface ComponentCustomProperties {
    $log: {
      trace: (module: LogModule, content: string, ...args: any[]) =&gt; void
      debug: (module: LogModule, content: string, ...args: any[]) =&gt; void
      info: (module: LogModule, content: string, ...args: any[]) =&gt; void
      warn: (module: LogModule, content: string, ...args: any[]) =&gt; void
      error: (module: LogModule, content: string, error?: unknown, ...args: any[]) =&gt; void
    }
  }
}

export {}
```

---

### Task 1.6: Rust数据库连接配置

**Files:**
- Create: `src-tauri/src/db.rs`
- Modify: `src-tauri/Cargo.toml` (添加依赖)
- Modify: `src-tauri/src/lib.rs` (导出模块)

**验收标准:** 实现SQLite连接池，初始化数据库实例，支持基本CRUD操作

- [ ] **Step 1: 修改 src-tauri/Cargo.toml 添加依赖**

在 `[dependencies]` 部分添加:

```toml
rusqlite = { version = "0.31", features = ["chrono"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
```

- [ ] **Step 2: 创建 src-tauri/src/db.rs**

```rust
//! 数据库模块 - SQLite连接与基础操作

use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;
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

pub type DbResult&lt;T&gt; = Result&lt;T, DbError&gt;;

/// 数据库管理器
pub struct DbManager {
    conn: Connection,
}

impl DbManager {
    /// 创建新的数据库管理器
    pub fn new(app_handle: &amp;tauri::AppHandle) -&gt; DbResult&lt;Self&gt; {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| DbError::Init(format!("无法获取应用数据目录: {}", e)))?;

        std::fs::create_dir_all(&amp;app_dir)?;

        let db_path = app_dir.join("dca_backtest.db");
        let conn = Connection::open(&amp;db_path)?;

        // 启用外键约束
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let manager = Self { conn };
        manager.init_tables()?;

        Ok(manager)
    }

    /// 获取数据库连接引用
    pub fn conn(&amp;self) -&gt; &amp;Connection {
        &amp;self.conn
    }

    /// 获取数据库连接的可变引用
    pub fn conn_mut(&amp;mut self) -&gt; &amp;mut Connection {
        &amp;mut self.conn
    }

    /// 初始化数据库表
    fn init_tables(&amp;self) -&gt; DbResult&lt;()&gt; {
        // 创建数据库版本表
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS db_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            );",
        )?;

        // 检查是否需要迁移
        let current_version: Option&lt;i32&gt; = self
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
    pub fn apply_migration(&amp;mut self, version: i32, sql: &amp;str) -&gt; DbResult&lt;()&gt; {
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
    pub fn get_current_version(&amp;self) -&gt; DbResult&lt;i32&gt; {
        Ok(self
            .conn
            .query_row("SELECT COALESCE(MAX(version), 0) FROM db_version", [], |row| {
                row.get(0)
            })?)
    }
}
```

- [ ] **Step 3: 修改 src-tauri/src/lib.rs 导出模块**

```rust
pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

---

### Task 1.7: 日志表数据库迁移

**Files:**
- Create: `migrations/20240416_create_app_log_table.sql`
- Modify: `src-tauri/src/db.rs` (集成迁移)

**验收标准:** 创建app_log表，包含id、level、module、content、timestamp、stack_trace字段

- [ ] **Step 1: 创建 migrations/20240416_create_app_log_table.sql**

```sql
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
WHERE datetime(timestamp) &gt;= datetime('now', '-30 days')
ORDER BY timestamp DESC;
```

- [ ] **Step 2: 修改 src-tauri/src/db.rs 添加迁移加载逻辑**

在 `DbManager` 中添加:

```rust
    /// 运行所有待应用的迁移
    pub fn run_migrations(&amp;mut self) -&gt; DbResult&lt;()&gt; {
        let current_version = self.get_current_version()?;

        // 迁移1: 创建日志表
        if current_version &lt; 1 {
            let migration_sql = include_str!("../../migrations/20240416_create_app_log_table.sql");
            self.apply_migration(1, migration_sql)?;
        }

        Ok(())
    }
```

并在 `init_tables` 后调用 `run_migrations`:

```rust
        let manager = Self { conn };
        manager.init_tables()?;
        // 这里不自动运行迁移，由显式调用
        Ok(manager)
```

---

### Task 1.8: 日志模块Rust后端实现

**Files:**
- Create: `src-tauri/src/log.rs`
- Modify: `src-tauri/src/lib.rs` (导出模块)

**验收标准:** 实现日志写入、查询、清理接口，支持5个日志级别，按模块分类

- [ ] **Step 1: 创建 src-tauri/src/log.rs**

```rust
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
    fn to_string(&amp;self) -&gt; String {
        match self {
            LogLevel::Trace =&gt; "trace".to_string(),
            LogLevel::Debug =&gt; "debug".to_string(),
            LogLevel::Info =&gt; "info".to_string(),
            LogLevel::Warn =&gt; "warn".to_string(),
            LogLevel::Error =&gt; "error".to_string(),
        }
    }
}

impl TryFrom&lt;&amp;str&gt; for LogLevel {
    type Error = String;

    fn try_from(s: &amp;str) -&gt; Result&lt;Self, Self::Error&gt; {
        match s.to_lowercase().as_str() {
            "trace" =&gt; Ok(LogLevel::Trace),
            "debug" =&gt; Ok(LogLevel::Debug),
            "info" =&gt; Ok(LogLevel::Info),
            "warn" =&gt; Ok(LogLevel::Warn),
            "error" =&gt; Ok(LogLevel::Error),
            _ =&gt; Err(format!("无效的日志级别: {}", s)),
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
    fn to_string(&amp;self) -&gt; String {
        match self {
            LogModule::Data =&gt; "data".to_string(),
            LogModule::Backtest =&gt; "backtest".to_string(),
            LogModule::Ui =&gt; "ui".to_string(),
            LogModule::Network =&gt; "network".to_string(),
            LogModule::System =&gt; "system".to_string(),
        }
    }
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Option&lt;i64&gt;,
    pub level: String,
    pub module: String,
    pub content: String,
    pub timestamp: String,
    pub stack_trace: Option&lt;String&gt;,
}

/// 日志查询参数
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogQueryParams {
    pub level: Option&lt;String&gt;,
    pub module: Option&lt;String&gt;,
    pub start_time: Option&lt;String&gt;,
    pub end_time: Option&lt;String&gt;,
    pub limit: Option&lt;i32&gt;,
    pub offset: Option&lt;i32&gt;,
}

/// 日志服务
pub struct LogService {
    db: DbManager,
}

impl LogService {
    /// 创建新的日志服务
    pub fn new(db: DbManager) -&gt; Self {
        Self { db }
    }

    /// 写入日志
    pub fn write_log(
        &amp;mut self,
        level: LogLevel,
        module: LogModule,
        content: String,
        stack_trace: Option&lt;String&gt;,
    ) -&gt; DbResult&lt;i64&gt; {
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
    pub fn query_logs(&amp;self, params: LogQueryParams) -&gt; DbResult&lt;Vec&lt;LogEntry&gt;&gt; {
        let mut sql = String::from(
            "SELECT id, level, module, content, timestamp, stack_trace
             FROM app_log WHERE 1=1",
        );
        let mut sql_params: Vec&lt;Box&lt;dyn rusqlite::ToSql&gt;&gt; = Vec::new();
        let mut param_index = 1;

        if let Some(level) = &amp;params.level {
            sql.push_str(&amp;format!(" AND level = ?{}", param_index));
            sql_params.push(Box::new(level.clone()));
            param_index += 1;
        }

        if let Some(module) = &amp;params.module {
            sql.push_str(&amp;format!(" AND module = ?{}", param_index));
            sql_params.push(Box::new(module.clone()));
            param_index += 1;
        }

        if let Some(start_time) = &amp;params.start_time {
            sql.push_str(&amp;format!(" AND timestamp &gt;= ?{}", param_index));
            sql_params.push(Box::new(start_time.clone()));
            param_index += 1;
        }

        if let Some(end_time) = &amp;params.end_time {
            sql.push_str(&amp;format!(" AND timestamp &lt;= ?{}", param_index));
            sql_params.push(Box::new(end_time.clone()));
            param_index += 1;
        }

        sql.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = params.limit {
            sql.push_str(&amp;format!(" LIMIT {}", limit));
        } else {
            sql.push_str(" LIMIT 100");
        }

        if let Some(offset) = params.offset {
            sql.push_str(&amp;format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db.conn().prepare(&amp;sql)?;
        let param_refs: Vec&lt;&amp;dyn rusqlite::ToSql&gt; = sql_params.iter().map(|p| p.as_ref()).collect();

        let logs = stmt.query_map(&amp;param_refs[..], |row| {
            Ok(LogEntry {
                id: Some(row.get(0)?),
                level: row.get(1)?,
                module: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
                stack_trace: row.get(5)?,
            })
        })?;

        logs.collect::&lt;Result&lt;Vec&lt;_&gt;, _&gt;&gt;().map_err(Into::into)
    }

    /// 清理旧日志（保留最近days天）
    pub fn clear_old_logs(&amp;mut self, days: i32) -&gt; DbResult&lt;usize&gt; {
        let deleted = self.db.conn().execute(
            "DELETE FROM app_log WHERE datetime(timestamp) &lt; datetime('now', ?1)",
            params![format!("-{} days", days)],
        )?;
        Ok(deleted)
    }

    /// 清空所有日志
    pub fn clear_all_logs(&amp;mut self) -&gt; DbResult&lt;usize&gt; {
        let deleted = self.db.conn().execute("DELETE FROM app_log", [])?;
        Ok(deleted)
    }
}
```

- [ ] **Step 2: 修改 src-tauri/src/lib.rs 导出日志模块**

```rust
pub mod db;
pub mod log;

// ... 其余保持不变
```

---

### Task 1.9: 注册日志Tauri命令

**Files:**
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/lib.rs` (添加状态管理和命令)

**验收标准:** 注册log_write、log_query、log_clear命令到Tauri

- [ ] **Step 1: 修改 src-tauri/src/lib.rs 完整实现**

```rust
pub mod db;
pub mod log;

use crate::db::DbManager;
use crate::log::{LogEntry, LogLevel, LogModule, LogQueryParams, LogService};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

/// 应用状态
struct AppState {
    db: Mutex&lt;Option&lt;DbManager&gt;&gt;,
    log_service: Mutex&lt;Option&lt;LogService&gt;&gt;,
}

/// Tauri命令响应格式
#[derive(Serialize)]
struct CommandResult&lt;T&gt; {
    ok: bool,
    data: Option&lt;T&gt;,
    error: Option&lt;String&gt;,
}

impl&lt;T&gt; CommandResult&lt;T&gt; {
    fn success(data: T) -&gt; Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(message: String) -&gt; Self {
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
    stack_trace: Option&lt;String&gt;,
}

/// 清理日志的参数
#[derive(Deserialize)]
struct ClearLogsParams {
    days: Option&lt;i32&gt;,
}

// ==================== Tauri Commands ====================

#[tauri::command]
async fn log_write(
    params: WriteLogParams,
    state: State&lt;'_, AppState&gt;,
) -&gt; CommandResult&lt;i64&gt; {
    let level = match LogLevel::try_from(params.level.as_str()) {
        Ok(l) =&gt; l,
        Err(e) =&gt; return CommandResult::error(e),
    };

    let module = match params.module.to_lowercase().as_str() {
        "data" =&gt; LogModule::Data,
        "backtest" =&gt; LogModule::Backtest,
        "ui" =&gt; LogModule::Ui,
        "network" =&gt; LogModule::Network,
        "system" =&gt; LogModule::System,
        _ =&gt; LogModule::System,
    };

    let mut log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_mut() {
        match service.write_log(level, module, params.content, params.stack_trace) {
            Ok(id) =&gt; CommandResult::success(id),
            Err(e) =&gt; CommandResult::error(e.to_string()),
        }
    } else {
        CommandResult::error("日志服务未初始化".to_string())
    }
}

#[tauri::command]
async fn log_query(
    params: LogQueryParams,
    state: State&lt;'_, AppState&gt;,
) -&gt; CommandResult&lt;Vec&lt;LogEntry&gt;&gt; {
    let log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_ref() {
        match service.query_logs(params) {
            Ok(logs) =&gt; CommandResult::success(logs),
            Err(e) =&gt; CommandResult::error(e.to_string()),
        }
    } else {
        CommandResult::error("日志服务未初始化".to_string())
    }
}

#[tauri::command]
async fn log_clear(
    params: ClearLogsParams,
    state: State&lt;'_, AppState&gt;,
) -&gt; CommandResult&lt;usize&gt; {
    let mut log_service = state.log_service.lock().unwrap();
    if let Some(service) = log_service.as_mut() {
        let result = if let Some(days) = params.days {
            service.clear_old_logs(days)
        } else {
            service.clear_all_logs()
        };
        match result {
            Ok(count) =&gt; CommandResult::success(count),
            Err(e) =&gt; CommandResult::error(e.to_string()),
        }
    } else {
        CommandResult::error("日志服务未初始化".to_string())
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
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 初始化数据库和日志服务
            let db = DbManager::new(app.handle())?;
            let mut log_service = LogService::new(db);

            // 运行迁移
            log_service.db.run_migrations()?;

            // 保存到状态
            let app_state = app.state::&lt;AppState&gt;();
            let db = LogService::new(DbManager::new(app.handle())?);
            *app_state.db.lock().unwrap() = Some(db.db);
            *app_state.log_service.lock().unwrap() = Some(log_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![log_write, log_query, log_clear])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

### Task 1.10: 日志模块前端SDK封装

**Files:**
- Create: `src/utils/log.ts`

**验收标准:** 封装trace/debug/info/warn/error方法，全局异常捕获自动上报日志

- [ ] **Step 1: 创建 src/utils/log.ts**

```typescript
/**
 * 日志模块前端SDK
 * 提供统一的日志上报接口，支持多级别、多模块分类
 */

import { invoke } from '@tauri-apps/api/core'
import type { LogLevel, LogModule, LogEntry, LogQueryParams } from '@/types/global'

// ==================== 类型定义 ====================

interface WriteLogOptions {
  level: LogLevel
  module: LogModule
  content: string
  stackTrace?: string
}

interface LogQueryOptions extends Partial&lt;LogQueryParams&gt; {}

// ==================== 内部工具函数 ====================

/**
 * 获取错误堆栈信息
 */
function getStackTrace(error?: unknown): string | undefined {
  if (error instanceof Error) {
    return error.stack
  }
  if (typeof error === 'string') {
    return error
  }
  // 生成当前调用堆栈
  try {
    throw new Error()
  } catch (e) {
    return (e as Error)?.stack
  }
}

/**
 * 格式化日志内容
 */
function formatContent(content: string, args: any[]): string {
  if (args.length === 0) {
    return content
  }
  return `${content} ${args.map(arg =&gt;
    typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
  ).join(' ')}`
}

// ==================== 日志SDK类 ====================

class Logger {
  private readonly buffer: WriteLogOptions[] = []
  private isFlushing = false
  private readonly maxBufferSize = 50
  private readonly flushInterval = 1000 // 1秒

  constructor() {
    // 定期刷新缓冲区
    setInterval(() =&gt; this.flush(), this.flushInterval)
  }

  /**
   * 写入日志到缓冲区
   */
  private async write(
    level: LogLevel,
    module: LogModule,
    content: string,
    stackTrace?: string
  ): Promise&lt;void&gt; {
    const logEntry: WriteLogOptions = {
      level,
      module,
      content,
      stackTrace
    }

    this.buffer.push(logEntry)

    // 缓冲区满或error级别立即刷新
    if (this.buffer.length &gt;= this.maxBufferSize || level === 'error') {
      await this.flush()
    }
  }

  /**
   * 刷新缓冲区，发送到Rust后端
   */
  private async flush(): Promise&lt;void&gt; {
    if (this.isFlushing || this.buffer.length === 0) {
      return
    }

    this.isFlushing = true
    const logs = [...this.buffer]
    this.buffer.length = 0

    try {
      // 逐个发送日志
      for (const log of logs) {
        await invoke('log_write', {
          params: {
            level: log.level,
            module: log.module,
            content: log.content,
            stackTrace: log.stackTrace
          }
        })
      }
    } catch (error) {
      console.error('日志发送失败:', error)
      // 失败时放回缓冲区
      this.buffer.unshift(...logs)
      if (this.buffer.length &gt; this.maxBufferSize * 2) {
        this.buffer.splice(this.maxBufferSize)
      }
    } finally {
      this.isFlushing = false
    }
  }

  // ==================== 公开方法 ====================

  /**
   * Trace级别日志 - 非常详细的调试信息
   */
  trace(module: LogModule, content: string, ...args: any[]): void {
    this.write('trace', module, formatContent(content, args))
    console.trace(`[${module}]`, content, ...args)
  }

  /**
   * Debug级别日志 - 调试信息
   */
  debug(module: LogModule, content: string, ...args: any[]): void {
    this.write('debug', module, formatContent(content, args))
    console.debug(`[${module}]`, content, ...args)
  }

  /**
   * Info级别日志 - 一般信息
   */
  info(module: LogModule, content: string, ...args: any[]): void {
    this.write('info', module, formatContent(content, args))
    console.info(`[${module}]`, content, ...args)
  }

  /**
   * Warn级别日志 - 警告信息
   */
  warn(module: LogModule, content: string, ...args: any[]): void {
    this.write('warn', module, formatContent(content, args))
    console.warn(`[${module}]`, content, ...args)
  }

  /**
   * Error级别日志 - 错误信息
   */
  error(module: LogModule, content: string, error?: unknown, ...args: any[]): void {
    const stackTrace = getStackTrace(error)
    const fullContent = error instanceof Error
      ? `${content}: ${error.message}`
      : formatContent(content, args)

    this.write('error', module, fullContent, stackTrace)
    console.error(`[${module}]`, content, error, ...args)
  }

  /**
   * 查询日志
   */
  async query(options?: LogQueryOptions): Promise&lt;LogEntry[]&gt; {
    const result = await invoke&lt;{ ok: boolean; data?: LogEntry[]; error?: string }&gt;('log_query', {
      params: options || {}
    })

    if (!result.ok) {
      throw new Error(result.error || '查询日志失败')
    }
    return result.data || []
  }

  /**
   * 清理日志
   * @param days 保留天数，不传则清空所有
   */
  async clear(days?: number): Promise&lt;number&gt; {
    const result = await invoke&lt;{ ok: boolean; data?: number; error?: string }&gt;('log_clear', {
      params: days ? { days } : {}
    })

    if (!result.ok) {
      throw new Error(result.error || '清理日志失败')
    }
    return result.data || 0
  }
}

// ==================== 全局实例 ====================

const logger = new Logger()

// ==================== 全局异常捕获 ====================

/**
 * 全局未捕获异常处理
 */
export function setupGlobalErrorHandlers(): void {
  // 未捕获的JavaScript错误
  window.addEventListener('error', (event) =&gt; {
    logger.error(
      'system',
      `全局未捕获错误: ${event.message}`,
      event.error || event,
      { filename: event.filename, lineno: event.lineno, colno: event.colno }
    )
  })

  // 未捕获的Promise rejection
  window.addEventListener('unhandledrejection', (event) =&gt; {
    logger.error(
      'system',
      '未处理的Promise Rejection',
      event.reason
    )
  })

  // Vue错误处理将在main.ts中设置
}

// ==================== 导出 ====================

export { logger }
export default logger

// 为了方便使用，导出便捷方法
export const log = {
  trace: logger.trace.bind(logger),
  debug: logger.debug.bind(logger),
  info: logger.info.bind(logger),
  warn: logger.warn.bind(logger),
  error: logger.error.bind(logger),
  query: logger.query.bind(logger),
  clear: logger.clear.bind(logger),
}
```

---

### Task 1.11 - 1.20: 后续任务框架

剩余任务(1.11-1.20)将按照相同的模式逐个完成，每个任务只修改单个文件。

关键文件清单:
- Task 1.11: `src/components/layout/TopNav.vue`
- Task 1.12: `src/components/layout/TabBar.vue`
- Task 1.13: `src/components/layout/Workspace.vue`
- Task 1.14: `src/components/common/Skeleton.vue`
- Task 1.15: `src/components/common/ProgressBar.vue`
- Task 1.16: `src/router/index.ts`
- Task 1.17: `src/App.vue`
- Task 1.18: `src/views/Dashboard.vue`
- Task 1.19: `src/components/common/Button.vue`
- Task 1.20: `src/components/common/Card.vue`

---

## 执行顺序提醒

⚠️ **必须严格按照TODO.md中的顺序执行:**
1. 每个任务只修改单个文件
2. 完成后必须按验收标准验证
3. 验证通过后更新TODO.md状态为✅
4. 然后才能开始下一个任务

---

## 验证清单

阶段1完成后需验证:
- [ ] Vite配置正确，路径别名生效
- [ ] Tailwind CSS主题完全符合UI_DESIGN.md
- [ ] SQLite数据库正常初始化，日志表创建成功
- [ ] 日志模块Rust后端正常工作
- [ ] 前端日志SDK可以正常上报和查询日志
- [ ] 全局异常捕获正常工作
- [ ] 所有基础UI组件渲染正确，样式符合设计规范
- [ ] 路由配置正确，页面可以正常切换
- [ ] `tauri dev` 可以正常启动，应用运行流畅
