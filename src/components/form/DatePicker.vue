<template>
  <div class="form-field">
    <label
      v-if="label"
      :for="dateId"
      class="field-label"
    >
      {{ label }}
    </label>

    <div class="date-input-wrapper">
      <input
        :id="dateId"
        type="date"
        :value="modelValue"
        :min="min"
        :max="max"
        :disabled="disabled"
        class="date-input"
        @input="handleInput"
      />
      <span v-if="errorMessage" class="error-text">
        {{ errorMessage }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useId } from 'vue'

interface Props {
  modelValue?: string
  label?: string
  min?: string
  max?: string
  disabled?: boolean
  errorMessage?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  label: '',
  min: '',
  max: '',
  disabled: false,
  errorMessage: '',
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const dateId = `date-${useId()}`

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement
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

.date-input-wrapper {
  position: relative;
}

.date-input {
  width: 100%;
  height: 36px;
  padding: 0 12px;
  border: 1px solid #E9ECEF;
  border-radius: 8px;
  background: #F8F9FA;
  font-size: 14px;
  color: #495057;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro', 'Inter', sans-serif;
  transition: all 0.15s ease;
}

.date-input:hover:not(:disabled) {
  border-color: #ADB5BD;
}

.date-input:focus {
  outline: none;
  border-color: #007AFF;
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.15);
}

.date-input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-text {
  position: absolute;
  bottom: -18px;
  left: 0;
  font-size: 10px;
  color: #FF3B30;
}
</style>
