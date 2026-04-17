<template>
  <div class="space-y-6 animate-fade-in">
    <!-- 欢迎标题 + 快捷操作 -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-display-lg text-neutral-500 font-bold">欢迎回来</h1>
        <p class="text-body text-neutral-400 mt-1">开始一个新的回测或查看历史记录</p>
      </div>
      <div class="flex space-x-3">
        <Button type="primary">
          <span class="flex items-center">
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
            </svg>
            新建回测
          </span>
        </Button>
      </div>
    </div>

    <!-- 数据概览卡片组 -->
    <div class="grid grid-cols-4 gap-4">
      <Card>
        <div class="text-caption text-neutral-300">总回测数</div>
        <div class="text-heading-1 text-neutral-500 font-bold mt-1">0</div>
      </Card>
      <Card>
        <div class="text-caption text-neutral-300">资产品种</div>
        <div class="text-heading-1 text-neutral-500 font-bold mt-1">0</div>
      </Card>
      <Card>
        <div class="text-caption text-neutral-300">总数据量</div>
        <div class="text-heading-1 text-neutral-500 font-bold mt-1">0</div>
      </Card>
      <Card>
        <div class="text-caption text-neutral-300">最近更新</div>
        <div class="text-heading-1 text-neutral-500 font-bold mt-1">-</div>
      </Card>
    </div>

    <!-- 最近回测记录 -->
    <Card>
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-heading-2 text-neutral-500 font-semibold">最近回测</h2>
        <Button type="text">查看全部</Button>
      </div>
      <div class="text-body text-neutral-300 text-center py-8">
        暂无回测记录，点击右上角"新建回测"开始
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import Card from '@/components/common/Card.vue'
import Button from '@/components/common/Button.vue'
import { log } from '../utils/log'

onMounted(() => {
  log.info('ui', 'Dashboard页面加载完成')
})
</script>
