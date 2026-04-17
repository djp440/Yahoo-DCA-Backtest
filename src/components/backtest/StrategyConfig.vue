<template>
  <div class="strategy-config space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-2">策略类型</label>
      <select
        :value="modelValue.strategy_type"
        @change="updateStrategyType"
        class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
      >
        <option value="fixed_invest">普通定投</option>
        <option value="value_averaging">价值平均策略</option>
      </select>
    </div>

    <!-- 普通定投参数 -->
    <template v-if="modelValue.strategy_type === 'fixed_invest'">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">每次投入金额</label>
        <div class="relative">
          <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">$</span>
          <input
            :value="params.invest_amount"
            @input="updateParam('invest_amount', ($event.target as HTMLInputElement).value)"
            type="number"
            min="0"
            step="100"
            class="w-full h-9 pl-7 pr-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
          />
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">定投频率</label>
        <select
          :value="modelValue.frequency"
          @change="updateFrequency"
          class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
        >
          <option value="weekly">每周</option>
          <option value="biweekly">双周</option>
          <option value="monthly">每月</option>
        </select>
      </div>
    </template>

    <!-- 价值平均策略参数 -->
    <template v-else-if="modelValue.strategy_type === 'value_averaging'">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">目标市值月增长率</label>
        <div class="relative">
          <input
            :value="(params.target_growth_rate || 0.02) * 100"
            @input="updateParam('target_growth_rate', (Number(($event.target as HTMLInputElement).value) / 100).toString())"
            type="number"
            min="0"
            max="100"
            step="0.1"
            class="w-full h-9 pl-3 pr-7 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
          />
          <span class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400">%</span>
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">初始市值</label>
        <div class="relative">
          <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">$</span>
          <input
            :value="params.initial_value"
            @input="updateParam('initial_value', ($event.target as HTMLInputElement).value)"
            type="number"
            min="0"
            step="1000"
            class="w-full h-9 pl-7 pr-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">最大单次投入</label>
          <div class="relative">
            <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">$</span>
            <input
              :value="params.max_investment"
              @input="updateParam('max_investment', ($event.target as HTMLInputElement).value)"
              type="number"
              min="0"
              class="w-full h-9 pl-7 pr-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
            />
          </div>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">最小单次投入</label>
          <div class="relative">
            <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">$</span>
            <input
              :value="params.min_investment"
              @input="updateParam('min_investment', ($event.target as HTMLInputElement).value)"
              type="number"
              min="0"
              class="w-full h-9 pl-7 pr-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface BacktestConfig {
  name: string
  symbols: Array<{ symbol: string; weight: number }>
  strategy_type: 'fixed_invest' | 'value_averaging'
  strategy_params: string
  start_date: string
  end_date: string
  frequency: 'weekly' | 'biweekly' | 'monthly'
}

const props = defineProps<{
  modelValue: BacktestConfig
}>()

const emit = defineEmits<{
  'update:modelValue': [value: BacktestConfig]
}>()

const params = computed(() => {
  try {
    return JSON.parse(props.modelValue.strategy_params || '{}')
  } catch {
    return {}
  }
})

function updateStrategyType(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', {
    ...props.modelValue,
    strategy_type: target.value as 'fixed_invest' | 'value_averaging',
  })
}

function updateFrequency(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', {
    ...props.modelValue,
    frequency: target.value as 'weekly' | 'biweekly' | 'monthly',
  })
}

function updateParam(key: string, value: string) {
  const newParams = { ...params.value, [key]: Number(value) }
  emit('update:modelValue', {
    ...props.modelValue,
    strategy_params: JSON.stringify(newParams),
  })
}
</script>