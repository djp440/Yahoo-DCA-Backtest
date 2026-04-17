/**
 * 自动数据更新模块
 * 应用启动时自动检查并更新本地缓存数据，不阻塞UI
 */

import { dataService } from '@/services/data'
import { logger } from '@/utils/log'

// 自动更新配置
const AUTO_UPDATE_CONFIG = {
  // 是否启用自动更新
  enabled: true,
  // 最大并发更新品种数
  maxConcurrent: 3,
  // 数据有效期天数（超过此天数未更新则触发更新）
  dataExpiryDays: 7,
}

export interface AutoUpdateOptions {
  enabled?: boolean
  maxConcurrent?: number
  dataExpiryDays?: number
}

export interface UpdateStatus {
  isRunning: boolean
  totalSymbols: number
  updatedCount: number
  failedCount: number
  errors: string[]
}

let currentStatus: UpdateStatus = {
  isRunning: false,
  totalSymbols: 0,
  updatedCount: 0,
  failedCount: 0,
  errors: [],
}

/**
 * 获取当前更新状态
 */
export function getUpdateStatus(): UpdateStatus {
  return { ...currentStatus }
}

/**
 * 检查数据是否需要更新
 */
export function needsUpdate(symbolMeta: { last_update: string | null }): boolean {
  if (!symbolMeta.last_update) return true

  const lastUpdate = new Date(symbolMeta.last_update)
  const now = new Date()
  const diffDays = Math.floor((now.getTime() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24))

  return diffDays >= AUTO_UPDATE_CONFIG.dataExpiryDays
}

/**
 * 获取默认结束日期（最新交易日）
 */
function getDefaultEndDate(): string {
  return new Date().toISOString().split('T')[0]
}

/**
 * 获取默认开始日期（3年前）
 */
function getDefaultStartDate(): string {
  const date = new Date()
  date.setFullYear(date.getFullYear() - 3)
  return date.toISOString().split('T')[0]
}

/**
 * 更新单个品种数据
 */
async function updateSingleSymbol(symbol: string): Promise<boolean> {
  try {
    const symbols = await dataService.listSymbols({ search: symbol, limit: 1 })
    const symbolMeta = symbols.find(s => s.symbol === symbol)

    const startDate = symbolMeta?.first_date || getDefaultStartDate()
    const endDate = getDefaultEndDate()

    const result = await dataService.fetchAndStore(symbol, startDate, endDate)
    logger.info('data', `自动更新 ${symbol} 成功，共 ${result.count} 条记录`)
    return true
  } catch (error) {
    const msg = `自动更新 ${symbol} 失败: ${error instanceof Error ? error.message : '未知错误'}`
    logger.error('data', msg, error)
    currentStatus.errors.push(msg)
    return false
  }
}

/**
 * 并发控制更新
 */
async function updateWithConcurrency(
  symbols: string[],
  maxConcurrent: number
): Promise<void> {
  const queue = [...symbols]
  const running: Promise<void>[] = []

  while (queue.length > 0 || running.length > 0) {
    // 填充运行中的槽位
    while (queue.length > 0 && running.length < maxConcurrent) {
      const symbol = queue.shift()!
      const promise = updateSingleSymbol(symbol).finally(() => {
        // 从运行中移除
        const index = running.indexOf(promise)
        if (index > -1) running.splice(index, 1)
        currentStatus.updatedCount++
      })
      running.push(promise)
    }

    // 等待任意一个完成
    if (running.length > 0) {
      await Promise.race(running)
    }
  }
}

/**
 * 执行自动更新
 */
export async function runAutoUpdate(options?: AutoUpdateOptions): Promise<UpdateStatus> {
  if (currentStatus.isRunning) {
    logger.warn('data', '自动更新已在运行中，跳过本次请求')
    return currentStatus
  }

  const config = { ...AUTO_UPDATE_CONFIG, ...options }

  if (!config.enabled) {
    logger.info('data', '自动更新已禁用')
    return currentStatus
  }

  currentStatus = {
    isRunning: true,
    totalSymbols: 0,
    updatedCount: 0,
    failedCount: 0,
    errors: [],
  }

  logger.info('data', '开始自动数据更新检查')

  try {
    // 获取本地缓存的品种列表
    const symbols = await dataService.listSymbols()

    if (symbols.length === 0) {
      logger.info('data', '本地无缓存品种，跳过自动更新')
      currentStatus.isRunning = false
      return currentStatus
    }

    currentStatus.totalSymbols = symbols.length

    // 筛选需要更新的品种
    const symbolsToUpdate = symbols
      .filter(needsUpdate)
      .map(s => s.symbol)

    if (symbolsToUpdate.length === 0) {
      logger.info('data', '所有品种数据均为最新，无需更新')
      currentStatus.isRunning = false
      return currentStatus
    }

    logger.info('data', `发现 ${symbolsToUpdate.length} 个品种需要更新`)

    // 执行更新
    await updateWithConcurrency(symbolsToUpdate, config.maxConcurrent)

    currentStatus.failedCount = symbolsToUpdate.length - currentStatus.updatedCount
    logger.info('data', `自动更新完成: 成功 ${currentStatus.updatedCount}, 失败 ${currentStatus.failedCount}`)

  } catch (error) {
    logger.error('data', '自动更新异常终止', error)
    currentStatus.errors.push(error instanceof Error ? error.message : '未知错误')
  } finally {
    currentStatus.isRunning = false
  }

  return currentStatus
}

/**
 * 停止自动更新
 */
export function cancelAutoUpdate(): void {
  if (currentStatus.isRunning) {
    logger.info('data', '请求取消自动更新')
    // 注意：由于使用Promise.race，无法真正中断正在执行的更新
    // 但可以将isRunning设为false以阻止新的更新开始
    currentStatus.isRunning = false
  }
}
