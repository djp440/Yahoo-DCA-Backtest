<template>
  <div class="backtest-result h-full flex flex-col">
    <!-- 头部操作区 -->
    <header class="flex items-center justify-between px-6 py-4 border-b border-gray-100">
      <div class="flex items-center gap-4">
        <button
          @click="$router.back()"
          class="w-8 h-8 flex items-center justify-center text-gray-400 hover:text-gray-600 transition-colors"
        >
          ←
        </button>
        <h1 class="text-xl font-semibold text-gray-800">{{ backtestName }}</h1>
      </div>
      <div class="flex gap-3">
        <button class="px-4 py-2 text-sm font-medium text-gray-600 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
          导出PDF
        </button>
        <button class="px-4 py-2 text-sm font-medium text-gray-600 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
          导出JSON
        </button>
        <button class="px-4 py-2 text-sm font-medium text-white bg-blue-500 rounded-lg hover:bg-blue-600 active:scale-95 transition-all">
          保存
        </button>
      </div>
    </header>

    <!-- 核心指标卡片 -->
    <div class="grid grid-cols-6 gap-4 p-6">
      <MetricCard
        label="总收益率"
        :value="metrics?.returns.total_return || 0"
        format="percent"
        :is-negative="(metrics?.returns.total_return || 0) < 0"
      />
      <MetricCard
        label="年化收益率"
        :value="metrics?.returns.annualized_return || 0"
        format="percent"
        :is-negative="(metrics?.returns.annualized_return || 0) < 0"
      />
      <MetricCard
        label="最大回撤"
        :value="metrics?.risk.max_drawdown || 0"
        format="percent"
        :is-negative="true"
      />
      <MetricCard
        label="夏普比率"
        :value="metrics?.risk_adjusted.sharpe_ratio || 0"
        format="ratio"
      />
      <MetricCard
        label="投入金额"
        :value="metrics?.dca.total_invested || 0"
        format="currency"
      />
      <MetricCard
        label="持仓市值"
        :value="metrics?.dca.market_value || 0"
        format="currency"
      />
    </div>

    <!-- 图表区域 -->
    <div class="flex-1 px-6 pb-4">
      <NetWorthChart
        v-if="dailyData.length > 0"
        :daily-data="dailyData"
        :height="400"
      />
    </div>

    <!-- 详细指标区域 -->
    <div class="p-6 pt-0">
      <MetricDetail
        v-if="metrics"
        :metrics="metrics"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import MetricCard from '@/components/backtest/MetricCard.vue'
import MetricDetail from '@/components/backtest/MetricDetail.vue'
import NetWorthChart from '@/components/charts/NetWorthChart.vue'
import type { BacktestMetrics, DailyData, BacktestResponse } from '@/services/backtest'

const route = useRoute()
const backtestName = ref('')
const metrics = ref<BacktestMetrics | null>(null)
const dailyData = ref<DailyData[]>([])

onMounted(() => {
  // 从路由参数或存储中获取回测结果
  const data = route.params.data
  if (data) {
    try {
      const result = JSON.parse(data as string) as BacktestResponse
      if (result.metrics) {
        metrics.value = result.metrics
      }
      if (result.daily_data) {
        dailyData.value = result.daily_data
      }
    } catch (e) {
      console.error('解析回测结果失败', e)
    }
  }
})
</script>

<style scoped>
.backtest-result {
  animation: fadeIn 0.25s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>