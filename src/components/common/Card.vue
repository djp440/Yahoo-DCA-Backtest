<template>
  <div :class="cardClasses">
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  hoverable?: boolean
  padding?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  hoverable: false,
  padding: true,
})

const cardClasses = computed(() => {
  const classes = [
    'bg-white rounded-card shadow-card',
    'transition-all duration-200',
  ]

  if (props.hoverable) {
    classes.push('hover:shadow-default hover:-translate-y-0.5')
  }

  if (props.padding) {
    classes.push('p-4')
  }

  return classes
})
</script>
