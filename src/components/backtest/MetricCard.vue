<template>
  <div
    :class="[
      'metric-card bg-white rounded-xl p-4 shadow-sm hover:shadow-md transition-all duration-200',
      'hover:-translate-y-0.5'
    ]"
  >
    <div class="text-sm text-gray-500 mb-1">{{ label }}</div>
    <div
      :class="[
        'text-2xl font-semibold tabular-nums',
        valueColorClass
      ]"
    >
      {{ displayValue }}
    </div>
    <div v-if="subValue" class="text-xs text-gray-400 mt-1">{{ subValue }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  label: string
  value: number | string
  format?: 'percent' | 'currency' | 'number' | 'ratio'
  subValue?: string
  isNegative?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  format: 'number',
  isNegative: false,
})

const displayValue = computed(() => {
  if (typeof props.value === 'string') return props.value

  switch (props.format) {
    case 'percent':
      return `${(props.value * 100).toFixed(2)}%`
    case 'currency':
      return `$${props.value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
    case 'ratio':
      return props.value.toFixed(2)
    default:
      return props.value.toLocaleString()
  }
})

const valueColorClass = computed(() => {
  if (props.format !== 'percent' && props.format !== 'ratio') {
    return 'text-gray-800'
  }

  if (props.isNegative) {
    return 'text-green-500' // 下跌色 (绿)
  }
  return 'text-red-500' // 上涨色 (红)
})
</script>

<style scoped>
.metric-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.metric-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}
</style>
