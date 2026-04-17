<template>
  <div ref="chartContainer" class="base-chart w-full h-full"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, shallowRef } from 'vue'
import { createChart, type IChartApi, type ISeriesApi, ColorType } from 'lightweight-charts'

// 图表颜色配置 - 符合UI设计规范
const chartColors = {
  netWorthLine: '#007AFF',      // 净值曲线 (主色)
  costLine: '#FF9500',          // 成本曲线 (橙色)
  drawdownLine: '#FF3B30',      // 回撤曲线 (红色)
  background: '#FFFFFF',        // 图表背景
  gridColor: '#E9ECEF',        // 网格线
  crosshairColor: '#ADB5BD',   // 十字线
}

interface Props {
  data?: any[]
  type?: 'line' | 'candlestick'
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  data: () => [],
  type: 'line',
  height: 400,
})

const chartContainer = ref<HTMLDivElement | null>(null)
const chart = shallowRef<IChartApi | null>(null)
const series = shallowRef<ISeriesApi | null>(null)

function initChart() {
  if (!chartContainer.value) return

  chart.value = createChart(chartContainer.value, {
    layout: {
      background: { type: ColorType.Solid, color: chartColors.background },
      textColor: '#495057',
      fontFamily: '-apple-system, BlinkMacSystemFont, "SF Pro", "Inter", sans-serif',
    },
    grid: {
      vertLines: { color: chartColors.gridColor },
      horzLines: { color: chartColors.gridColor },
    },
    crosshair: {
      vertLine: { color: chartColors.crosshairColor, labelBackgroundColor: '#007AFF' },
      horzLine: { color: chartColors.crosshairColor, labelBackgroundColor: '#007AFF' },
    },
    rightPriceScale: {
      borderColor: chartColors.gridColor,
    },
    timeScale: {
      borderColor: chartColors.gridColor,
      timeVisible: true,
    },
    handleScale: {
      axisPressedMouseMove: true,
    },
    handleScroll: {
      mouseWheel: true,
      pressedMouseMove: true,
    },
  })

  if (props.type === 'line') {
    series.value = chart.value.addLineSeries({
      color: chartColors.netWorthLine,
      lineWidth: 2,
      priceFormat: {
        type: 'price',
        precision: 2,
      },
    })
  }

  if (props.data.length > 0 && series.value) {
    series.value.setData(props.data)
  }

  chart.value.timeScale().fitContent()
}

onMounted(() => {
  initChart()

  // 响应式调整
  const resizeObserver = new ResizeObserver((entries) => {
    if (entries.length === 0 || !chart.value) return
    const { width, height } = entries[0].contentRect
    chart.value.applyOptions({ width, height })
  })

  if (chartContainer.value) {
    resizeObserver.observe(chartContainer.value)
  }
})

onUnmounted(() => {
  if (chart.value) {
    chart.value.remove()
    chart.value = null
  }
})

watch(() => props.data, (newData) => {
  if (series.value && newData.length > 0) {
    series.value.setData(newData)
    chart.value?.timeScale().fitContent()
  }
}, { deep: true })

defineExpose({ chart, series })
</script>

<style scoped>
.base-chart {
  min-height: 300px;
}
</style>
