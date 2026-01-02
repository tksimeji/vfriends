<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const toggle = () => {
  if (props.disabled) return;
  emit('update:modelValue', !props.modelValue);
};
</script>

<template>
  <button
    type="button"
    role="switch"
    :aria-checked="props.modelValue"
    :aria-disabled="props.disabled"
    class="border border-vrc-highlight/40 h-6 inline-flex items-center relative rounded-full transition-colors w-11"
    :class="[
      props.modelValue ? 'bg-vrc-highlight/60' : 'bg-vrc-background-secondary',
      props.disabled ? 'opacity-50' : 'hover:border-vrc-highlight/70',
    ]"
    @click="toggle"
  >
    <span
      class="absolute -translate-y-1/2 bg-white left-1 rounded-full shadow size-4 top-1/2 transition-transform"
      :class="props.modelValue ? 'translate-x-5' : 'translate-x-0'"
    />
  </button>
</template>

