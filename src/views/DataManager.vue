<template>
  <div class="animate-fade-in p-6">
    <!-- 页面标题与操作按钮 -->
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-heading-1 text-neutral-500 font-semibold">数据管理</h1>
      <div class="flex gap-3">
        <Button variant="secondary" @click="handleUpdateAll" :loading="isUpdatingAll">
          更新全部
        </Button>
        <Button variant="primary" @click="showAddModal = true">
          添加品种
        </Button>
      </div>
    </div>

    <!-- 搜索与筛选 -->
    <div class="flex gap-4 mb-4">
      <div class="flex-1">
        <Input
          v-model="searchKeyword"
          placeholder="搜索品种代码或名称..."
          label="搜索"
        />
      </div>
      <div class="w-40">
        <Select
          v-model="typeFilter"
          :options="typeOptions"
          placeholder="全部类型"
          label="类型筛选"
        />
      </div>
    </div>

    <!-- 数据概览 -->
    <div class="grid grid-cols-4 gap-4 mb-6">
      <Card v-for="stat in statsCards" :key="stat.label">
        <div class="text-center">
          <div class="text-2xl font-bold text-[#007AFF]">{{ stat.value }}</div>
          <div class="text-xs text-[#ADB5BD] mt-1">{{ stat.label }}</div>
        </div>
      </Card>
    </div>

    <!-- 品种数据表格 -->
    <Card :padding="false">
      <VirtualTable
        :columns="tableColumns"
        :data="filteredSymbols"
        :height="400"
        row-key="symbol"
        @row-click="handleRowClick"
      >
        <template #actions="{ row }">
          <div class="flex gap-2">
            <Button variant="text" size="small" @click.stop="handleUpdate(row)">
              更新
            </Button>
            <Button variant="text" size="small" danger @click.stop="handleDelete(row)">
              删除
            </Button>
          </div>
        </template>

        <template #empty>
          <div class="py-12 text-center">
            <p class="text-body text-[#ADB5BD] mb-4">暂无数据</p>
            <Button variant="primary" @click="showAddModal = true">
              添加第一个品种
            </Button>
          </div>
        </template>
      </VirtualTable>
    </Card>

    <!-- 添加品种弹窗 -->
    <div
      v-if="showAddModal"
      class="modal-overlay animate-fade-in"
      @click.self="showAddModal = false"
    >
      <div class="modal-content animate-spring">
        <div class="modal-header">
          <h2 class="text-heading-2 text-neutral-500 font-semibold">添加品种</h2>
          <button class="modal-close" @click="showAddModal = false">&times;</button>
        </div>

        <div class="modal-body">
          <Input
            v-model="addForm.symbol"
            label="品种代码"
            placeholder="如: AAPL, SPY, BTC-USD"
            required
            :error="!!addError"
            :error-message="addError"
          />

          <div class="flex gap-3 mt-4">
            <div class="flex-1">
              <DatePicker
                v-model="addForm.startDate"
                label="开始日期"
              />
            </div>
            <div class="flex-1">
              <DatePicker
                v-model="addForm.endDate"
                label="结束日期"
              />
            </div>
          </div>

          <div v-if="isDownloading" class="mt-4">
            <div class="flex items-center gap-2">
              <div class="spinner" />
              <span class="text-sm text-[#ADB5BD]">正在下载数据...</span>
            </div>
            <div class="progress-bar mt-2">
              <div class="progress-fill" :style="{ width: downloadProgress + '%' }" />
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <Button variant="secondary" @click="showAddModal = false">取消</Button>
          <Button
            variant="primary"
            :loading="isDownloading"
            @click="handleAdd"
          >
            确认添加
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import Card from '@/components/common/Card.vue'
import Button from '@/components/common/Button.vue'
import Input from '@/components/form/Input.vue'
import Select from '@/components/form/Select.vue'
import DatePicker from '@/components/form/DatePicker.vue'
import VirtualTable from '@/components/common/VirtualTable.vue'
import { dataService } from '@/services/data'
import type { SymbolMeta } from '@/types/global'
import { logger } from '@/utils/log'

// ==================== 状态 ====================

const symbols = ref<SymbolMeta[]>([])
const searchKeyword = ref('')
const typeFilter = ref('')
const showAddModal = ref(false)
const isDownloading = ref(false)
const isUpdatingAll = ref(false)
const downloadProgress = ref(0)
const addError = ref('')

const addForm = ref({
  symbol: '',
  startDate: getDefaultStartDate(),
  endDate: getDefaultEndDate(),
})

// ==================== 计算属性 ====================

const typeOptions = computed(() => {
  const types = [...new Set(symbols.value.map(s => s.type))]
  return [
    { label: '全部类型', value: '' },
    ...types.map(t => ({ label: t, value: t })),
  ]
})

const filteredSymbols = computed(() => {
  let result = symbols.value

  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase()
    result = result.filter(
      s => s.symbol.toLowerCase().includes(keyword) ||
           s.name.toLowerCase().includes(keyword)
    )
  }

  if (typeFilter.value) {
    result = result.filter(s => s.type === typeFilter.value)
  }

  return result
})

const statsCards = computed(() => [
  { label: '品种总数', value: symbols.value.length.toString() },
  { label: '数据记录', value: formatNumber(symbols.value.reduce((sum, s) => sum + s.total_records, 0)) },
  { label: '最早数据', value: getEarliestDate() },
  { label: '最近更新', value: getLatestUpdate() },
])

const tableColumns = [
  { key: 'symbol', title: '品种代码', width: '100px' },
  { key: 'name', title: '名称', width: '150px' },
  { key: 'type', title: '类型', width: '80px' },
  { key: 'dateRange', title: '数据范围', width: '180px' },
  { key: 'total_records', title: '记录数', width: '80px' },
  { key: 'last_update', title: '最后更新', width: '120px' },
  { key: 'actions', title: '操作', width: '120px' },
]

// ==================== 生命周期 ====================

onMounted(async () => {
  await loadSymbols()
})

// ==================== 方法 ====================

async function loadSymbols() {
  try {
    symbols.value = await dataService.listSymbols()
  } catch (error) {
    logger.error('data', '加载品种列表失败', error)
  }
}

async function handleAdd() {
  if (!addForm.value.symbol.trim()) {
    addError.value = '请输入品种代码'
    return
  }

  addError.value = ''
  isDownloading.value = true
  downloadProgress.value = 0

  try {
    const result = await dataService.fetchAndStore(
      addForm.value.symbol.trim().toUpperCase(),
      addForm.value.startDate,
      addForm.value.endDate
    )

    downloadProgress.value = 100
    logger.info('data', `成功下载 ${result.symbol} 数据，共 ${result.count} 条记录`)

    showAddModal.value = false
    addForm.value.symbol = ''
    addForm.value.startDate = getDefaultStartDate()
    addForm.value.endDate = getDefaultEndDate()

    await loadSymbols()
  } catch (error) {
    addError.value = error instanceof Error ? error.message : '下载失败'
    logger.error('data', '下载品种数据失败', error)
  } finally {
    isDownloading.value = false
  }
}

async function handleUpdate(row: SymbolMeta) {
  try {
    const startDate = row.first_date || getDefaultStartDate()
    const endDate = getDefaultEndDate()

    await dataService.fetchAndStore(row.symbol, startDate, endDate)
    logger.info('data', `更新 ${row.symbol} 数据成功`)
    await loadSymbols()
  } catch (error) {
    logger.error('data', `更新 ${row.symbol} 数据失败`, error)
  }
}

async function handleUpdateAll() {
  if (symbols.value.length === 0) return

  isUpdatingAll.value = true
  try {
    for (const symbol of symbols.value) {
      const startDate = symbol.first_date || getDefaultStartDate()
      const endDate = getDefaultEndDate()
      await dataService.fetchAndStore(symbol.symbol, startDate, endDate)
    }
    logger.info('data', '更新全部品种数据完成')
    await loadSymbols()
  } catch (error) {
    logger.error('data', '批量更新数据失败', error)
  } finally {
    isUpdatingAll.value = false
  }
}

async function handleDelete(row: SymbolMeta) {
  if (!confirm(`确认删除 ${row.symbol} 及其所有数据？此操作不可撤销。`)) {
    return
  }

  try {
    await dataService.deleteSymbol(row.symbol)
    logger.info('data', `已删除品种 ${row.symbol}`)
    await loadSymbols()
  } catch (error) {
    logger.error('data', `删除 ${row.symbol} 失败`, error)
  }
}

function handleRowClick(row: Record<string, any>) {
  logger.info('ui', `点击品种: ${row.symbol}`)
}

// ==================== 工具函数 ====================

function getDefaultStartDate(): string {
  const date = new Date()
  date.setFullYear(date.getFullYear() - 3)
  return formatDate(date)
}

function getDefaultEndDate(): string {
  return formatDate(new Date())
}

function formatDate(date: Date): string {
  return date.toISOString().split('T')[0]
}

function formatNumber(n: number): string {
  if (n >= 10000) {
    return (n / 10000).toFixed(1) + '万'
  }
  return n.toLocaleString()
}

function getEarliestDate(): string {
  const dates = symbols.value
    .map(s => s.first_date)
    .filter(Boolean) as string[]
  if (dates.length === 0) return '-'
  return dates.sort()[0]?.substring(0, 10) || '-'
}

function getLatestUpdate(): string {
  const dates = symbols.value
    .map(s => s.last_update)
    .filter(Boolean) as string[]
  if (dates.length === 0) return '-'
  return dates.sort().reverse()[0]?.substring(0, 10) || '-'
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: #FFFFFF;
  border-radius: 16px;
  box-shadow: 0 12px 48px rgba(0, 0, 0, 0.16);
  width: 480px;
  max-width: 90vw;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid #E9ECEF;
}

.modal-close {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  font-size: 20px;
  color: #ADB5BD;
  cursor: pointer;
  border-radius: 50%;
  transition: all 0.15s ease;
}

.modal-close:hover {
  background: #F8F9FA;
  color: #495057;
}

.modal-body {
  padding: 24px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid #E9ECEF;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #E9ECEF;
  border-top-color: #007AFF;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.progress-bar {
  height: 4px;
  background: #E9ECEF;
  border-radius: 2px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: #007AFF;
  border-radius: 2px;
  transition: width 0.3s ease;
}
</style>
