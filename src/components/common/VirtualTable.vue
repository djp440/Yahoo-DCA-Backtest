<template>
  <div class="virtual-table" :style="{ height: height + 'px' }">
    <!-- 表头 -->
    <div class="table-header">
      <div
        v-for="col in columns"
        :key="col.key"
        class="header-cell"
        :style="{ width: getColumnWidth(col), minWidth: getColumnWidth(col) }"
      >
        {{ col.title }}
      </div>
    </div>

    <!-- 表格内容区域 -->
    <div
      ref="scrollContainer"
      class="table-body"
      :style="{ height: bodyHeight + 'px' }"
      @scroll="handleScroll"
    >
      <!-- 撑开滚动区域 -->
      <div :style="{ height: totalHeight + 'px', position: 'relative' }">
        <!-- 可见区域的数据行 -->
        <div
          v-for="row in visibleRows"
          :key="getRowKey(row)"
          class="table-row"
          :style="{
            transform: `translateY(${getTop(row)}px)`,
            position: 'absolute',
            width: '100%',
          }"
          :class="{ 'row-hover': !disableHover }"
          @click="handleRowClick(row)"
        >
          <div
            v-for="col in columns"
            :key="col.key"
            class="table-cell"
            :style="{ width: getColumnWidth(col), minWidth: getColumnWidth(col) }"
          >
            <slot
              :name="col.key"
              :row="row"
              :value="row[col.key]"
            >
              {{ row[col.key] }}
            </slot>
          </div>
        </div>

        <!-- 空状态 -->
        <div v-if="data.length === 0" class="empty-state">
          <slot name="empty">
            <p class="text-body text-[#ADB5BD]">暂无数据</p>
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface Column {
  key: string
  title: string
  width?: string
}

interface Props {
  columns: Column[]
  data: Record<string, any>[]
  rowKey?: string
  height?: number
  rowHeight?: number
  disableHover?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  rowKey: 'id',
  height: 400,
  rowHeight: 40,
  disableHover: false,
})

const emit = defineEmits<{
  'row-click': [row: Record<string, any>]
}>()

const scrollContainer = ref<HTMLElement | null>(null)
const scrollTop = ref(0)

// 表头高度
const headerHeight = 36

// 表格体高度
const bodyHeight = computed(() => props.height - headerHeight)

// 总行数
const totalRows = computed(() => props.data.length)

// 总高度
const totalHeight = computed(() => totalRows.value * props.rowHeight)

// 可见行数
const visibleCount = computed(() => {
  return Math.ceil(bodyHeight.value / props.rowHeight) + 2
})

// 起始行索引
const startIndex = computed(() => {
  return Math.floor(scrollTop.value / props.rowHeight)
})

// 可见数据行
const visibleRows = computed(() => {
  const start = Math.max(0, startIndex.value)
  const end = Math.min(totalRows.value, start + visibleCount.value)
  return props.data.slice(start, end)
})

// 获取行顶部位置
function getTop(row: Record<string, any>): number {
  const index = props.data.indexOf(row)
  return index * props.rowHeight
}

// 获取行唯一标识
function getRowKey(row: Record<string, any>): string {
  return String(row[props.rowKey]) || String(props.data.indexOf(row))
}

// 获取列宽度
function getColumnWidth(col: Column): string {
  return col.width || '1fr'
}

// 滚动处理
function handleScroll(event: Event) {
  const target = event.target as HTMLElement
  scrollTop.value = target.scrollTop
}

// 行点击
function handleRowClick(row: Record<string, any>) {
  emit('row-click', row)
}

onMounted(() => {
  // initialized
})
</script>

<style scoped>
.virtual-table {
  display: flex;
  flex-direction: column;
  border: 1px solid #E9ECEF;
  border-radius: 8px;
  overflow: hidden;
  background: #FFFFFF;
}

.table-header {
  display: flex;
  height: 36px;
  background: #F8F9FA;
  border-bottom: 1px solid #E9ECEF;
  flex-shrink: 0;
}

.header-cell {
  font-size: 12px;
  font-weight: 600;
  color: #495057;
  padding: 0 12px;
  display: flex;
  align-items: center;
}

.table-body {
  overflow-y: auto;
  overflow-x: hidden;
}

.table-row {
  display: flex;
  height: 40px;
  border-bottom: 1px solid #F8F9FA;
  transition: background-color 0.15s ease;
}

.table-row.row-hover:hover {
  background-color: #F8F9FA;
}

.table-cell {
  font-size: 14px;
  color: #495057;
  padding: 0 12px;
  display: flex;
  align-items: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}
</style>
