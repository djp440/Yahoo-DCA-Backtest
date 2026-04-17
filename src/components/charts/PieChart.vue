<template>
  <div class="pie-chart">
    <h3 class="text-lg font-medium text-gray-800 mb-4">{{ title }}</h3>
    <div class="flex items-center gap-8">
      <!-- 饼图 -->
      <div class="relative" :style="{ width: `${size}px`, height: `${size}px` }">
        <svg :width="size" :height="size" class="transform -rotate-90">
          <circle
            v-for="(segment, index) in segments"
            :key="index"
            cx="50%"
            cy="50%"
            :r="radius"
            fill="transparent"
            :stroke="segment.color"
            :stroke-width="strokeWidth"
            :stroke-dasharray="segment.dashArray"
            :stroke-dashoffset="segment.dashOffset"
            class="transition-all duration-300"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="text-center">
            <div class="text-2xl font-semibold text-gray-800">{{ total.toFixed(1)}}%</div>
            <div class="text-xs text-gray-400">总计</div>
          </div>
        </div>
      </div>

      <!-- 图例 -->
      <div class="space-y-2">
        <div
          v-for="(item, index) in data"
          :key="index"
          class="flex items-center gap-2"
        >
          <span
            class="w-3 h-3 rounded-sm"
            :style="{ backgroundColor: item.color }"
          ></span>
          <span class="text-sm text-gray-600">{{ item.label }}</span>
          <span class="text-sm font-medium text-gray-800">{{ item.value.toFixed(1) }}%</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface PieData {
  label: string
  value: number
  color: string
}

interface Props {
  data: PieData[]
  title?: string
  size?: number
}

const props = withDefaults(defineProps<Props>(), {
  title: '资产配置',
  size: 200,
})

const radius = computed(() => (props.size / 2) - 20)
const strokeWidth = 30

const total = computed(() => props.data.reduce((sum, item) => sum + item.value, 0))

const segments = computed(() => {
  let offset = 0
  const circumference = 2 * Math.PI * radius.value

  return props.data.map(item => {
    const percentage = item.value / 100
    const dashArray = `${circumference * percentage} ${circumference * (1 - percentage)}`
    const dashOffset = -offset * circumference / 100
    offset += item.value

    return {
      color: item.color,
      dashArray,
      dashOffset,
    }
  })
})
</script>