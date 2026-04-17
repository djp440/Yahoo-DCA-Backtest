<template>
  <div class="animate-pulse-soft">
    <div v-if="type === 'card'" class="space-y-3">
      <div class="h-4 bg-neutral-200 rounded w-3/4 animate-pulse-soft"></div>
      <div class="h-3 bg-neutral-200 rounded w-full animate-pulse-soft"></div>
      <div class="h-3 bg-neutral-200 rounded w-5/6 animate-pulse-soft"></div>
    </div>
    <div v-else-if="type === 'list'" class="space-y-4">
      <div v-for="i in 5" :key="i" class="flex items-center space-x-3">
        <div class="w-10 h-10 bg-neutral-200 rounded-full animate-pulse-soft"></div>
        <div class="flex-1 space-y-2">
          <div class="h-3 bg-neutral-200 rounded w-1/2 animate-pulse-soft"></div>
          <div class="h-3 bg-neutral-200 rounded w-3/4 animate-pulse-soft"></div>
        </div>
      </div>
    </div>
    <div v-else-if="type === 'chart'" class="space-y-3">
      <div class="h-32 bg-neutral-200 rounded-card animate-pulse-soft"></div>
      <div class="flex justify-between">
        <div class="h-3 bg-neutral-200 rounded w-20 animate-pulse-soft"></div>
        <div class="h-3 bg-neutral-200 rounded w-20 animate-pulse-soft"></div>
      </div>
    </div>
    <div v-else :style="{ width, height }" class="bg-neutral-200 rounded animate-pulse-soft"></div>
  </div>
</template>

<script setup lang="ts">
type SkeletonType = 'card' | 'list' | 'chart' | 'custom'

interface Props {
  type?: SkeletonType
  width?: string
  height?: string
}

withDefaults(defineProps<Props>(), {
  type: 'custom',
  width: '100%',
  height: '20px',
})
</script>
