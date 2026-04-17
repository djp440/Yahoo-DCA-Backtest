<template>
  <div class="w-full">
    <div v-if="showLabel" class="flex justify-between mb-1">
      <span class="text-body text-neutral-400">{{ label }}</span>
      <span v-if="percentage !== null" class="text-body font-medium text-neutral-500">{{ percentage }}%</span>
    </div>
    <div class="relative h-2 bg-neutral-200 rounded-full overflow-hidden">
      <div
        :class="progressBarClasses"
        :style="{ width: progressWidth }"
        class="h-full rounded-full transition-all duration-300"
      ></div>
      <div v-if="indeterminate" class="absolute inset-0 overflow-hidden">
        <div class="absolute inset-0 -translate-x-full bg-gradient-to-r from-transparent via-white/30 to-transparent animate-[progress-indeterminate_1.5s_infinite]"></div>
      </div>
    </div>
    <button
      v-if="showCancel && onCancel"
      @click="onCancel"
      class="mt-2 text-caption text-danger hover:text-danger/80 transition-colors"
    >
      取消
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  percentage?: number | null
  label?: string
  showLabel?: boolean
  type?: 'primary' | 'success' | 'warning' | 'danger'
  indeterminate?: boolean
  showCancel?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  percentage: null,
  label: '',
  showLabel: true,
  type: 'primary',
  indeterminate: false,
  showCancel: false,
})

const emit = defineEmits<{
  cancel: []
}>()

const progressWidth = computed(() => {
  if (props.indeterminate) return '60%'
  if (props.percentage === null) return '0%'
  return `${Math.min(100, Math.max(0, props.percentage))}%`
})

const progressBarClasses = computed(() => {
  const typeColors: Record<string, string> = {
    primary: 'bg-primary',
    success: 'bg-success',
    warning: 'bg-warning',
    danger: 'bg-danger',
  }
  return [typeColors[props.type]]
})

function onCancel() {
  emit('cancel')
}
</script>

<style>
@keyframes progress-indeterminate {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(200%);
  }
}
</style>
