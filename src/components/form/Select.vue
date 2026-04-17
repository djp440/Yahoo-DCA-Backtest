<template>
  <div class="form-field">
    <label
      v-if="label"
      :for="selectId"
      class="field-label"
    >
      {{ label }}
    </label>

    <div class="select-wrapper">
      <select
        :id="selectId"
        :value="modelValue"
        :disabled="disabled"
        class="select-input"
        @change="handleChange"
      >
        <option v-if="placeholder" value="" disabled>
          {{ placeholder }}
        </option>
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
        >
          {{ option.label }}
        </option>
      </select>
      <span class="select-arrow">
        <svg width="12" height="8" viewBox="0 0 12 8" fill="none">
          <path d="M1 1.5L6 6.5L11 1.5" stroke="#495057" stroke-width="1.5"
                stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </span>
      <span v-if="errorMessage" class="error-text">
        {{ errorMessage }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useId } from 'vue'

interface SelectOption {
  label: string
  value: string
}

interface Props {
  modelValue?: string
  label?: string
  placeholder?: string
  options: SelectOption[]
  disabled?: boolean
  errorMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: '',
  placeholder: '',
  disabled: false,
  errorMessage: '',
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const selectId = `select-${useId()}`

function handleChange(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', target.value)
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

.select-wrapper {
  position: relative;
}

.select-input {
  width: 100%;
  height: 36px;
  padding: 0 32px 0 12px;
  border: 1px solid #E9ECEF;
  border-radius: 8px;
  background: #F8F9FA;
  font-size: 14px;
  color: #495057;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro', 'Inter', sans-serif;
  appearance: none;
  cursor: pointer;
  transition: all 0.15s ease;
}

.select-input:hover:not(:disabled) {
  border-color: #ADB5BD;
}

.select-input:focus {
  outline: none;
  border-color: #007AFF;
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.15);
}

.select-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.select-arrow {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  pointer-events: none;
}

.error-text {
  position: absolute;
  bottom: -18px;
  left: 0;
  font-size: 10px;
  color: #FF3B30;
}
</style>
