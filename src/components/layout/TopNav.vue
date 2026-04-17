<template>
  <header class="h-14 bg-white border-b border-neutral-200 flex items-center justify-between px-4">
    <!-- Logo -->
    <div class="flex items-center">
      <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center mr-3">
        <span class="text-white font-bold text-sm">DCA</span>
      </div>
      <span class="text-heading-3 text-neutral-500 font-semibold">DCA回测工具</span>
    </div>

    <!-- 主菜单 -->
    <nav class="flex items-center space-x-1">
      <button
        v-for="item in menuItems"
        :key="item.key"
        :class="[
          'px-4 py-2 rounded-lg text-body transition-all duration-150',
          activeMenu === item.key
            ? 'bg-primary-light text-primary font-medium'
            : 'text-neutral-400 hover:text-neutral-500 hover:bg-neutral-100'
        ]"
        @click="handleMenuClick(item.key)"
      >
        {{ item.label }}
      </button>
    </nav>

    <!-- 全局操作区 -->
    <div class="flex items-center space-x-2">
      <button class="p-2 text-neutral-300 hover:text-neutral-400 hover:bg-neutral-100 rounded-lg transition-all duration-150">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
        </svg>
      </button>
      <button class="p-2 text-neutral-300 hover:text-neutral-400 hover:bg-neutral-100 rounded-lg transition-all duration-150">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface MenuItem {
  key: string
  label: string
}

const menuItems: MenuItem[] = [
  { key: 'backtest', label: '回测' },
  { key: 'data', label: '数据' },
  { key: 'history', label: '历史' },
]

const activeMenu = ref('backtest')

const emit = defineEmits<{
  menuChange: [key: string]
}>()

function handleMenuClick(key: string) {
  activeMenu.value = key
  emit('menuChange', key)
}
</script>
