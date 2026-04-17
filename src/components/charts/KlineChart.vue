<template>
  <div class="kline-chart">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-medium text-gray-800">K线图表</h3>
      <div class="flex gap-2">
        <button
          v-for="indicator in indicators"
          :key="indicator"
          @click="toggleIndicator(indicator)"
          :class="[
            'px-3 py-1 text-sm rounded-lg transition-colors',
            activeIndicators.includes(indicator)
              ? 'bg-blue-100 text-blue-600'
              : 'text-gray-500 hover:bg-gray-100'
          ]"
        >
          {{ indicator }}
        </button>
      </div>
    </div>

    <div ref="chartContainer" class="bg-white rounded-xl shadow-sm p-4" :style="{ height: `${height}px` }"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { createChart, ColorType } from 'lightweight-charts'

interface CandleData {
  time: string
  open: number
  high: number
  low: number
  close: number
  volume?: number
}

interface Props {
  data: CandleData[]
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  data: () => [],
  height: 500,
})

const chartContainer = ref<HTMLDivElement | null>(null)
const chart = ref<any>(null)
const candleSeries = ref<any>(null)
const volumeSeries = ref<any>(null)

const indicators = ['MA5', 'MA20', 'MA60', 'RSI', 'ATR']
const activeIndicators = ref<string[]>([])

// K线颜色配置 - 符合UI设计规范 (A股习惯：红涨绿跌)
const upColor = '#FF3B30'      // 上涨K线 (红涨)
const downColor = '#34C759'     // 下跌K线 (绿跌)

function initChart() {
  if (!chartContainer.value) return

  chart.value = createChart(chartContainer.value, {
    layout: {
      background: { type: ColorType.Solid, color: '#FFFFFF' },
      textColor: '#495057',
    },
    grid: {
      vertLines: { color: '#E9ECEF' },
      horzLines: { color: '#E9ECEF' },
    },
    rightPriceScale: {
      borderColor: '#E9ECEF',
    },
    timeScale: {
      borderColor: '#E9ECEF',
      timeVisible: true,
    },
  })

  candleSeries.value = chart.value.addCandlestickSeries({
    upColor: upColor,
    downColor: downColor,
    wickUpColor: upColor,
    wickDownColor: downColor,
  })

  volumeSeries.value = chart.value.addHistogramSeries({
    color: '#007AFF80',
    priceFormat: {
      type: 'volume',
    },
    priceScaleId: 'volume',
  })

  chart.value.priceScale('volume').applyOptions({
    scaleMargins: {
      top: 0.8,
      bottom: 0,
    },
  })

  if (props.data.length > 0) {
    candleSeries.value.setData(props.data)
    volumeSeries.value.setData(
      props.data.map(d => ({
        time: d.time,
        value: d.volume || 0,
        color: d.close >= d.open ? upColor : downColor,
      }))
    )
  }

  chart.value.timeScale().fitContent()
}

function toggleIndicator(indicator: string) {
  const index = activeIndicators.value.indexOf(indicator)
  if (index > -1) {
    activeIndicators.value.splice(index, 1)
  } else {
    activeIndicators.value.push(indicator)
  }
}

onMounted(() => {
  initChart()
})

onUnmounted(() => {
  if (chart.value) {
    chart.value.remove()
    chart.value = null
  }
})

watch(() => props.data, (newData) => {
  if (candleSeries.value && newData.length > 0) {
    candleSeries.value.setData(newData)
  }
}, { deep: true })
</script>