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

type ButtonVariant = 'primary' | 'secondary' | 'text' | 'danger'
type ButtonSize = 'small' | 'medium' | 'large'

interface Props {
  variant?: ButtonVariant
  size?: ButtonSize
  danger?: boolean
  disabled?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'medium',
  danger: false,
  disabled: false,
  loading: false,
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClasses = computed(() => {
  const baseClasses = [
    'rounded-button font-medium transition-all duration-150',
    'focus:outline-none focus:ring-2 focus:ring-primary/50',
    'disabled:opacity-50 disabled:cursor-not-allowed',
  ]

  const sizeClasses: Record<ButtonSize, string[]> = {
    small: ['px-2 py-1 text-xs'],
    medium: ['px-4 py-2'],
    large: ['px-6 py-3 text-lg'],
  }

  const typeClasses: Record<ButtonVariant, string[]> = {
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

  const variant = props.danger ? 'danger' : props.variant

  return [...baseClasses, ...sizeClasses[props.size], ...typeClasses[variant]]
})

function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>
