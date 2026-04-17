<template>
  <div class="backtest-config h-full flex">
    <!-- 侧边步骤栏 -->
    <aside class="w-48 bg-white border-r border-gray-200 p-4">
      <div class="space-y-2">
        <div
          v-for="(step, index) in steps"
          :key="index"
          :class="[
            'flex items-center gap-3 p-3 rounded-lg cursor-pointer transition-all duration-200',
            currentStep === index
              ? 'bg-blue-50 text-blue-600'
              : 'text-gray-500 hover:bg-gray-50'
          ]"
          @click="currentStep = index"
        >
          <span
            :class="[
              'w-6 h-6 rounded-full flex items-center justify-center text-sm font-medium',
              currentStep > index
                ? 'bg-green-500 text-white'
                : currentStep === index
                ? 'bg-blue-500 text-white'
                : 'bg-gray-200 text-gray-500'
            ]"
          >
            {{ currentStep > index ? '✓' : index + 1 }}
          </span>
          <span class="text-sm font-medium">{{ step }}</span>
        </div>
      </div>
    </aside>

    <!-- 主配置区 -->
    <main class="flex-1 p-6 overflow-auto">
      <!-- 步骤1: 基础配置 -->
      <div v-show="currentStep === 0" class="space-y-6">
        <h2 class="text-2xl font-semibold text-gray-800">基础配置</h2>

        <div class="bg-white rounded-xl shadow-sm p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">回测名称</label>
            <input
              v-model="config.name"
              type="text"
              class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
              placeholder="请输入回测名称"
            />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">开始日期</label>
              <input
                v-model="config.start_date"
                type="date"
                class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">结束日期</label>
              <input
                v-model="config.end_date"
                type="date"
                class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">定投频率</label>
            <select
              v-model="config.frequency"
              class="w-full h-9 px-3 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none transition-all"
            >
              <option value="weekly">每周</option>
              <option value="biweekly">双周</option>
              <option value="monthly">每月</option>
            </select>
          </div>
        </div>
      </div>

      <!-- 步骤2: 品种配置 -->
      <div v-show="currentStep === 1" class="space-y-6">
        <h2 class="text-2xl font-semibold text-gray-800">品种配置</h2>
        <div class="bg-white rounded-xl shadow-sm p-6">
          <p class="text-gray-500">品种选择组件将在这里渲染</p>
        </div>
      </div>

      <!-- 步骤3: 策略配置 -->
      <div v-show="currentStep === 2" class="space-y-6">
        <h2 class="text-2xl font-semibold text-gray-800">策略配置</h2>
        <div class="bg-white rounded-xl shadow-sm p-6">
          <p class="text-gray-500">策略参数配置组件将在这里渲染</p>
        </div>
      </div>

      <!-- 步骤4: 高级配置 -->
      <div v-show="currentStep === 3" class="space-y-6">
        <h2 class="text-2xl font-semibold text-gray-800">高级配置</h2>
        <div class="bg-white rounded-xl shadow-sm p-6">
          <p class="text-gray-500">高级配置选项</p>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="flex justify-between mt-6 pt-6 border-t border-gray-200">
        <button
          v-if="currentStep > 0"
          @click="currentStep--"
          class="px-4 py-2 text-sm font-medium text-gray-600 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
        >
          上一步
        </button>
        <div class="flex gap-3 ml-auto">
          <button
            class="px-4 py-2 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors"
          >
            保存模板
          </button>
          <button
            @click="startBacktest"
            class="px-6 py-2 text-sm font-medium text-white bg-blue-500 rounded-lg hover:bg-blue-600 active:scale-95 transition-all"
          >
            开始回测
          </button>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { backtestService, type BacktestConfig } from '@/services/backtest'
import { logger } from '@/utils/log'

const router = useRouter()

const steps = ['基础配置', '品种配置', '策略配置', '高级配置']
const currentStep = ref(0)

const config = reactive<BacktestConfig>({
  name: '',
  symbols: [],
  strategy_type: 'fixed_invest',
  strategy_params: '{}',
  start_date: '2021-01-01',
  end_date: '2024-12-31',
  frequency: 'monthly',
})

async function startBacktest() {
  logger.info('backtest', '开始回测')
  try {
    const result = await backtestService.run(config)
    if (result.success) {
      router.push({
        name: 'BacktestResult',
        params: { data: JSON.stringify(result) },
      })
    }
  } catch (error) {
    logger.error('backtest', `回测失败: ${error}`)
  }
}
</script>

<style scoped>
.backtest-config {
  animation: fadeIn 0.25s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>