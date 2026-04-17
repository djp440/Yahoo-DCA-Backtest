<template>
  <div class="h-11 bg-neutral-100 border-b border-neutral-200 flex items-center px-2">
    <div class="flex items-center space-x-1 overflow-x-auto scrollbar-hide">
      <div
        v-for="tab in tabs"
        :key="tab.id"
        :class="[
          'flex items-center px-3 py-1.5 rounded-button text-sm cursor-pointer transition-all duration-150 group',
          activeTab === tab.id
            ? 'bg-white text-neutral-500 shadow-sm'
            : 'text-neutral-300 hover:text-neutral-400 hover:bg-white/50'
        ]"
        @click="handleTabClick(tab.id)"
      >
        <span class="truncate max-w-[150px]">{{ tab.title }}</span>
        <button
          v-if="tabs.length > 1"
          class="ml-2 w-4 h-4 rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 hover:bg-neutral-200 transition-all duration-150"
          @click.stop="handleCloseTab(tab.id)"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
    </div>
    <button
      class="ml-2 p-1.5 text-neutral-300 hover:text-neutral-400 hover:bg-white/50 rounded-button transition-all duration-150"
      @click="handleNewTab"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Tab {
  id: string
  title: string
  type: 'backtest' | 'data' | 'result'
}

const tabs = ref<Tab[]>([
  { id: 'tab-1', title: '回测配置', type: 'backtest' },
])

const activeTab = ref('tab-1')

const emit = defineEmits<{
  tabChange: [id: string]
  newTab: []
  closeTab: [id: string]
}>()

let tabCounter = 1

function handleTabClick(id: string) {
  activeTab.value = id
  emit('tabChange', id)
}

function handleNewTab() {
  tabCounter++
  const newTab: Tab = {
    id: `tab-${tabCounter}`,
    title: '新建回测',
    type: 'backtest',
  }
  tabs.value.push(newTab)
  activeTab.value = newTab.id
  emit('newTab')
}

function handleCloseTab(id: string) {
  const index = tabs.value.findIndex(t => t.id === id)
  if (index === -1) return

  tabs.value.splice(index, 1)

  if (activeTab.value === id && tabs.value.length > 0) {
    const newIndex = Math.min(index, tabs.value.length - 1)
    activeTab.value = tabs.value[newIndex].id
    emit('tabChange', activeTab.value)
  }

  emit('closeTab', id)
}
</script>
