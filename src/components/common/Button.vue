<template>
  <button
    :class="buttonClasses"
    :disabled="disabled || loading"
    @click="handleClick"
  >
    <span v-if="loading" class="inline-block w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2"></span>
    <slot />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type ButtonType = 'primary' | 'secondary' | 'text' | 'danger'

interface Props {
  type?: ButtonType
  disabled?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'primary',
  disabled: false,
  loading: false,
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClasses = computed(() => {
  const baseClasses = [
    'px-4 py-2 rounded-button font-medium transition-all duration-150',
    'focus:outline-none focus:ring-2 focus:ring-primary/50',
    'disabled:opacity-50 disabled:cursor-not-allowed',
  ]

  const typeClasses: Record<ButtonType, string[]> = {
    primary: [
      'bg-primary text-white',
      'hover:bg-primary-dark active:bg-primary-dark/90',
    ],
    secondary: [
      'bg-white border-2 border-primary text-primary',
      'hover:bg-primary-light active:bg-primary-light/80',
    ],
    text: [
      'text-primary bg-transparent',
      'hover:bg-neutral-100 active:bg-neutral-200',
    ],
    danger: [
      'bg-danger text-white',
      'hover:bg-danger/90 active:bg-danger/80',
    ],
  }

  return [...baseClasses, ...typeClasses[props.type]]
})

function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>
