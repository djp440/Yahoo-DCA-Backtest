<template>
  <div class="form-field">
    <label
      v-if="label"
      :for="inputId"
      class="field-label"
    >
      {{ label }}
      <span v-if="required" class="required-mark">*</span>
    </label>

    <div class="input-wrapper">
      <input
        :id="inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :readonly="readonly"
        :class="inputClasses"
        @input="handleInput"
        @focus="handleFocus"
        @blur="handleBlur"
      />

      <slot name="suffix" />

      <span v-if="errorMessage" class="error-text">
        {{ errorMessage }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, useId } from 'vue'

interface Props {
  modelValue?: string | number
  label?: string
  type?: string
  placeholder?: string
  disabled?: boolean
  readonly?: boolean
  required?: boolean
  error?: boolean
  errorMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: '',
  type: 'text',
  placeholder: '',
  disabled: false,
  readonly: false,
  required: false,
  error: false,
  errorMessage: '',
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const inputId = `input-${useId()}`

const inputClasses = computed(() => {
  const classes = [
    'w-full h-9 px-3 rounded-input border text-body',
    'transition-all duration-150',
    'bg-[#F8F9FA]',
  ]

  if (props.disabled) {
    classes.push('opacity-50 cursor-not-allowed border-[#E9ECEF]')
  } else if (props.error || props.errorMessage) {
    classes.push('border-[#FF3B30] focus:border-[#FF3B30]')
  } else {
    classes.push('border-[#E9ECEF]')
    classes.push('focus:border-[#007AFF] focus:shadow-[0_0_0_2px_rgba(0,122,255,0.15)]')
    classes.push('hover:border-[#ADB5BD]')
  }

  return classes.join(' ')
})

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}

function handleFocus() {
  // reserved for future use
}

function handleBlur() {
  // reserved for future use
}
</script>

<style scoped>
.form-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 12px;
  font-weight: 600;
  color: #495057;
}

.required-mark {
  color: #FF3B30;
  margin-left: 2px;
}

.input-wrapper {
  position: relative;
}

.error-text {
  position: absolute;
  bottom: -18px;
  left: 0;
  font-size: 10px;
  color: #FF3B30;
}
</style>
