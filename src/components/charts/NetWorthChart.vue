<template>
  <div class="net-worth-chart">
    <div class="flex items-center gap-4 mb-4">
      <h3 class="text-lg font-medium text-gray-800">净值曲线</h3>
      <div class="flex gap-2">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          @click="activeTab = tab.key"
          :class="[
            'px-3 py-1 text-sm rounded-lg transition-colors',
            activeTab === tab.key
              ? 'bg-blue-100 text-blue-600'
              : 'text-gray-500 hover:bg-gray-100'
          ]"
        >
          {{ tab.label }}
        </button>
      </div>
    </div>

    <div class="relative bg-white rounded-xl shadow-sm p-4" :style="{ height: `${height}px` }">
      <BaseChart
        v-if="activeTab === 'networth'"
        ref="netWorthChartRef"
        :data="netWorthData"
        type="line"
      />
      <BaseChart
        v-else-if="activeTab === 'drawdown'"
        ref="drawdownChartRef"
        :data="drawdownData"
        type="line"
      />
      <BaseChart
        v-else
        ref="costChartRef"
        :data="costData"
        type="line"
      />

      <!-- 图例 -->
      <div class="absolute top-4 right-4 flex gap-4 text-sm">
        <div class="flex items-center gap-2">
          <span class="w-3 h-0.5 bg-blue-500 rounded"></span>
          <span class="text-gray-500">{{ activeTab === 'networth' ? '净值' : activeTab === 'drawdown' ? '回撤' : '成本' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import BaseChart from './BaseChart.vue'
import type { DailyData } from '@/services/backtest'

interface Props {
  dailyData: DailyData[]
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  height: 400,
})

const tabs = [
  { key: 'networth', label: '净值' },
  { key: 'cost', label: '成本' },
  { key: 'drawdown', label: '回撤' },
]

const activeTab = ref('networth')
const netWorthChartRef = ref<InstanceType<typeof BaseChart> | null>(null)
const costChartRef = ref<InstanceType<typeof BaseChart> | null>(null)
const drawdownChartRef = ref<InstanceType<typeof BaseChart> | null>(null)

const netWorthData = computed(() => {
  return props.dailyData.map(d => ({
    time: d.date,
    value: d.value,
  }))
})

const costData = computed(() => {
  return props.dailyData.map(d => ({
    time: d.date,
    value: d.cost,
  }))
})

const drawdownData = computed(() => {
  return props.dailyData.map(d => ({
    time: d.date,
    value: d.drawdown * 100, // 转为百分比
  }))
})
</script>