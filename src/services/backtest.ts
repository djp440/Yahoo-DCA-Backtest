/**
 * 回测服务 - 前端封装
 * 封装回测相关的Tauri命令调用
 */

import { invoke } from '@tauri-apps/api/core'
import { logger } from '@/utils/log'

// 回测配置接口
export interface SymbolConfig {
  symbol: string
  weight: number
}

export interface BacktestConfig {
  name: string
  symbols: SymbolConfig[]
  strategy_type: 'fixed_invest' | 'value_averaging'
  strategy_params: string
  start_date: string
  end_date: string
  frequency: 'weekly' | 'biweekly' | 'monthly'
}

export interface BacktestRequest {
  config: BacktestConfig
}

export interface DailyData {
  date: string
  price: number
  value: number
  cost: number
  drawdown: number
}

export interface ReturnMetrics {
  total_return: number
  annualized_return: number
  final_value: number
  total_invested: number
  profit: number
}

export interface RiskMetrics {
  max_drawdown: number
  max_drawdown_duration: number
  volatility: number
  downside_deviation: number
  var_95: number
  cvar_95: number
}

export interface RiskAdjustedMetrics {
  sharpe_ratio: number
  sortino_ratio: number
  calmar_ratio: number
  omega_ratio: number
}

export interface DCAMetrics {
  total_invested: number
  market_value: number
  invest_count: number
  avg_cost: number
  dca_return: number
  lump_sum_return: number
  breakeven_point: number
}

export interface BacktestMetrics {
  returns: ReturnMetrics
  risk: RiskMetrics
  risk_adjusted: RiskAdjustedMetrics
  dca: DCAMetrics
}

export interface BacktestResponse {
  success: boolean
  record_id?: number
  metrics?: BacktestMetrics
  daily_data?: DailyData[]
  error?: string
}

export interface BacktestStatus {
  isRunning: boolean
  progress: number
  currentStep: string
}

/**
 * 运行回测
 */
export async function runBacktest(config: BacktestConfig): Promise<BacktestResponse> {
  logger.info('backtest', `启动回测: ${config.name}`)

  try {
    const request: BacktestRequest = { config }
    const result = await invoke<string>('backtest_run', { request })
    const response = JSON.parse(result) as BacktestResponse

    if (response.success) {
      logger.info('backtest', `回测完成: ${config.name}`)
    } else {
      logger.error('backtest', `回测失败: ${response.error}`)
    }

    return response
  } catch (error) {
    logger.error('backtest', `回测异常: ${error}`)
    throw error
  }
}

/**
 * 查询回测记录
 */
export async function queryBacktest(recordId: number): Promise<BacktestResponse> {
  try {
    const result = await invoke<string>('backtest_query', { recordId })
    return JSON.parse(result) as BacktestResponse
  } catch (error) {
    logger.error('backtest', `查询回测记录失败: ${error}`)
    throw error
  }
}

/**
 * 删除回测记录
 */
export async function deleteBacktest(recordId: number): Promise<boolean> {
  try {
    await invoke('backtest_delete', { recordId })
    logger.info('backtest', `删除回测记录: ${recordId}`)
    return true
  } catch (error) {
    logger.error('backtest', `删除回测记录失败: ${error}`)
    throw error
  }
}

// 回测服务单例
export const backtestService = {
  run: runBacktest,
  query: queryBacktest,
  delete: deleteBacktest,
}
