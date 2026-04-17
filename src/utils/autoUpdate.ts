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
function needsUpdate(symbolMeta: { last_update: string | null }): boolean {
  if (!symbolMeta.last_update) return true

  const lastUpdate = new Date(symbolMeta.last_update)
  const now = new Date()
  const diffDays = Math.floor((now.getTime() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24))

  return diffDays >= AUTO_UPDATE_CONFIG.dataExpiryDays
}
