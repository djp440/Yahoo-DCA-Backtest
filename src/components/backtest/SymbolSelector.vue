<template>
  <div class="symbol-selector">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-medium text-gray-800">已选品种</h3>
      <button
        @click="showAddModal = true"
        class="px-3 py-1.5 text-sm font-medium text-blue-500 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors"
      >
        + 添加品种
      </button>
    </div>

    <div v-if="selectedSymbols.length === 0" class="text-center py-8 text-gray-400">
      暂未添加品种，请点击"添加品种"开始
    </div>

    <div v-else class="space-y-2">
      <div
        v-for="(item, index) in selectedSymbols"
        :key="item.symbol"
        class="flex items-center gap-4 p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
      >
        <div class="flex-1">
          <span class="font-medium text-gray-800">{{ item.symbol }}</span>
          <span class="ml-2 text-sm text-gray-400">{{ item.name }}</span>
        </div>
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-500">权重</span>
          <input
            v-model.number="item.weight"
            type="number"
            min="0"
            max="100"
            class="w-20 h-8 px-2 text-sm text-center rounded border border-gray-200 focus:border-blue-500 outline-none"
          />
          <span class="text-sm text-gray-500">%</span>
        </div>
        <button
          @click="removeSymbol(index)"
          class="w-8 h-8 flex items-center justify-center text-gray-400 hover:text-red-500 transition-colors"
        >
          ✕
        </button>
      </div>
    </div>

    <!-- 添加品种弹窗 -->
    <Teleport to="body">
      <div
        v-if="showAddModal"
        class="fixed inset-0 bg-black/40 flex items-center justify-center z-50"
        @click.self="showAddModal = false"
      >
        <div class="bg-white rounded-2xl shadow-xl w-[500px] max-h-[80vh] overflow-hidden animate-spring">
          <div class="p-6 border-b border-gray-100">
            <h3 class="text-lg font-semibold text-gray-800">添加品种</h3>
          </div>
          <div class="p-6">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索品种代码或名称..."
              class="w-full h-10 px-4 rounded-lg border border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-100 outline-none"
            />
            <div class="mt-4 max-h-64 overflow-auto space-y-1">
              <div
                v-for="symbol in filteredSymbols"
                :key="symbol.symbol"
                @click="addSymbol(symbol)"
                class="p-3 rounded-lg cursor-pointer hover:bg-blue-50 transition-colors"
              >
                <span class="font-medium text-gray-800">{{ symbol.symbol }}</span>
                <span class="ml-2 text-sm text-gray-400">{{ symbol.name }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Symbol {
  symbol: string
  name: string
}

const selectedSymbols = defineModel<Array<{ symbol: string; name: string; weight: number }>>('symbols', {
  default: () => [],
})

const showAddModal = ref(false)
const searchQuery = ref('')

const availableSymbols: Symbol[] = [
  { symbol: 'SPY', name: '标普500 ETF' },
  { symbol: 'QQQ', name: '纳斯达克100 ETF' },
  { symbol: 'VTI', name: 'Total Stock Market ETF' },
  { symbol: 'TLT', name: '美国20年期以上国债 ETF' },
  { symbol: 'GLD', name: '黄金 ETF' },
]

const filteredSymbols = computed(() => {
  if (!searchQuery.value) return availableSymbols
  const query = searchQuery.value.toLowerCase()
  return availableSymbols.filter(
    s => s.symbol.toLowerCase().includes(query) || s.name.toLowerCase().includes(query)
  )
})

function addSymbol(symbol: Symbol) {
  if (!selectedSymbols.value.find(s => s.symbol === symbol.symbol)) {
    selectedSymbols.value.push({ ...symbol, weight: 50 })
  }
  showAddModal.value = false
}

function removeSymbol(index: number) {
  selectedSymbols.value.splice(index, 1)
}
</script>

<style scoped>
.animate-spring {
  animation: spring 0.3s ease-out;
}

@keyframes spring {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>