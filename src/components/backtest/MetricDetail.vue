<template>
  <div class="metric-detail bg-white rounded-xl shadow-sm p-6">
    <div class="grid grid-cols-2 gap-6">
      <!-- 收益指标 -->
      <div>
        <h4 class="text-sm font-medium text-gray-500 mb-4">收益指标</h4>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">总收益率</span>
            <span :class="metrics.returns.total_return >= 0 ? 'text-red-500' : 'text-green-500'">
              {{ formatPercent(metrics.returns.total_return) }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">年化收益率</span>
            <span :class="metrics.returns.annualized_return >= 0 ? 'text-red-500' : 'text-green-500'">
              {{ formatPercent(metrics.returns.annualized_return) }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">复利终值</span>
            <span class="text-gray-800">{{ formatCurrency(metrics.returns.final_value) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">绝对收益</span>
            <span :class="metrics.returns.profit >= 0 ? 'text-red-500' : 'text-green-500'">
              {{ formatCurrency(metrics.returns.profit) }}
            </span>
          </div>
        </div>
      </div>

      <!-- 风险指标 -->
      <div>
        <h4 class="text-sm font-medium text-gray-500 mb-4">风险指标</h4>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">最大回撤</span>
            <span class="text-red-500">{{ formatPercent(metrics.risk.max_drawdown) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">波动率</span>
            <span class="text-gray-800">{{ formatPercent(metrics.risk.volatility) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">VaR (95%)</span>
            <span class="text-gray-800">{{ formatPercent(metrics.risk.var_95) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">CVaR (95%)</span>
            <span class="text-gray-800">{{ formatPercent(metrics.risk.cvar_95) }}</span>
          </div>
        </div>
      </div>

      <!-- 风险调整收益 -->
      <div>
        <h4 class="text-sm font-medium text-gray-500 mb-4">风险调整收益</h4>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">夏普比率</span>
            <span class="text-gray-800">{{ metrics.risk_adjusted.sharpe_ratio.toFixed(2) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">索提诺比率</span>
            <span class="text-gray-800">{{ metrics.risk_adjusted.sortino_ratio.toFixed(2) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">卡玛比率</span>
            <span class="text-gray-800">{{ metrics.risk_adjusted.calmar_ratio.toFixed(2) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">欧米伽比率</span>
            <span class="text-gray-800">{{ metrics.risk_adjusted.omega_ratio.toFixed(2) }}</span>
          </div>
        </div>
      </div>

      <!-- 定投指标 -->
      <div>
        <h4 class="text-sm font-medium text-gray-500 mb-4">定投指标</h4>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">投入本金</span>
            <span class="text-gray-800">{{ formatCurrency(metrics.dca.total_invested) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">持仓市值</span>
            <span class="text-gray-800">{{ formatCurrency(metrics.dca.market_value) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">平均成本</span>
            <span class="text-gray-800">{{ formatCurrency(metrics.dca.avg_cost) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">投入次数</span>
            <span class="text-gray-800">{{ metrics.dca.invest_count }}次</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BacktestMetrics } from '@/services/backtest'

interface Props {
  metrics: BacktestMetrics
}

defineProps<Props>()

function formatPercent(value: number): string {
  return `${(value * 100).toFixed(2)}%`
}

function formatCurrency(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
}
</script>
