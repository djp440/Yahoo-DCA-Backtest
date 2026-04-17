/**
 * 日志模块前端SDK
 * 提供统一的日志上报接口，支持多级别、多模块分类
 */

import { invoke } from '@tauri-apps/api/core'
import type { LogLevel, LogModule, LogEntry, LogQueryParams } from '../types/global'

// ==================== 类型定义 ====================

interface WriteLogOptions {
  level: LogLevel
  module: LogModule
  content: string
  stackTrace?: string
}

interface LogQueryOptions extends Partial<LogQueryParams> {}

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
  return `${content} ${args.map(arg =>
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
    setInterval(() => this.flush(), this.flushInterval)
  }

  /**
   * 写入日志到缓冲区
   */
  private async write(
    level: LogLevel,
    module: LogModule,
    content: string,
    stackTrace?: string
  ): Promise<void> {
    const logEntry: WriteLogOptions = {
      level,
      module,
      content,
      stackTrace
    }

    this.buffer.push(logEntry)

    // 缓冲区满或error级别立即刷新
    if (this.buffer.length >= this.maxBufferSize || level === 'error') {
      await this.flush()
    }
  }

  /**
   * 刷新缓冲区，发送到Rust后端
   */
  private async flush(): Promise<void> {
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
      if (this.buffer.length > this.maxBufferSize * 2) {
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
  async query(options?: LogQueryOptions): Promise<LogEntry[]> {
    const result = await invoke<{ ok: boolean; data?: LogEntry[]; error?: string }>('log_query', {
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
  async clear(days?: number): Promise<number> {
    const result = await invoke<{ ok: boolean; data?: number; error?: string }>('log_clear', {
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
  window.addEventListener('error', (event) => {
    logger.error(
      'system',
      `全局未捕获错误: ${event.message}`,
      event.error || event,
      { filename: event.filename, lineno: event.lineno, colno: event.colno }
    )
  })

  // 未捕获的Promise rejection
  window.addEventListener('unhandledrejection', (event) => {
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
