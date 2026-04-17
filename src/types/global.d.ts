/**
 * 全局TypeScript类型定义
 */

// ==================== 通用API响应类型 ====================
export interface ApiResponse<T = any> {
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
export interface TauriCommandResult<T = any> {
  ok: boolean
  data?: T
  error?: string
}

// ==================== 通用工具类型 ====================
export type Nullable<T> = T | null

export type Optional<T> = T | undefined

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P]
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
      trace: (module: LogModule, content: string, ...args: any[]) => void
      debug: (module: LogModule, content: string, ...args: any[]) => void
      info: (module: LogModule, content: string, ...args: any[]) => void
      warn: (module: LogModule, content: string, ...args: any[]) => void
      error: (module: LogModule, content: string, error?: unknown, ...args: any[]) => void
    }
  }
}

export {}
