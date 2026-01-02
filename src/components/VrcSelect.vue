<script setup lang="ts">
import {ChevronDownIcon} from 'lucide-vue-next';
import {computed, onBeforeUnmount, onMounted, ref} from 'vue';

type Option = {
  value: string;
  label: string;
};

const props = withDefaults(defineProps<{
  label?: string;
  helper?: string;
  disabled?: boolean;
  modelValue: string;
  options: Option[];
}>(), {
  label: '',
  helper: '',
  disabled: false,
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const isOpen = ref(false);
const rootRef = ref<HTMLElement | null>(null);

const selectedOption = computed(
  () => props.options.find((option) => option.value === props.modelValue) ?? null,
);

const toggleOpen = () => {
  if (props.disabled) return;
  isOpen.value = !isOpen.value;
};

const close = () => {
  isOpen.value = false;
};

const selectOption = (value: string) => {
  emit('update:modelValue', value);
  close();
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    close();
  }
  if (event.key === 'Enter' || event.key === ' ') {
    event.preventDefault();
    toggleOpen();
  }
};

const handleClickOutside = (event: MouseEvent) => {
  const root = rootRef.value;
  if (!root) return;
  if (!(event.target instanceof Node)) return;
  if (!root.contains(event.target)) {
    close();
  }
};

const triggerClasses = computed(() => {
  const base =
    'bg-vrc-button/80 border-2 border-vrc-highlight/20 flex items-center justify-between px-3 py-2 rounded-md text-left text-sm text-vrc-text transition w-full';
  const state = props.disabled
    ? 'border-vrc-highlight/10 cursor-not-allowed text-vrc-text/40'
    : 'hover:border-vrc-highlight/60';
  const focus = 'focus-visible:border-vrc-highlight/70 focus-visible:ring-2 focus-visible:ring-vrc-highlight/40';
  return `${base} ${state} ${focus}`;
});

const menuClasses = computed(() => {
  const base =
    'absolute bg-vrc-background-secondary border border-vrc-highlight/20 left-0 max-h-60 mt-2 overflow-y-auto rounded-md shadow-xl w-full z-50';
  const state = props.disabled ? 'pointer-events-none opacity-50' : '';
  return `${base} ${state}`;
});

onMounted(() => {
  window.addEventListener('mousedown', handleClickOutside);
});

onBeforeUnmount(() => {
  window.removeEventListener('mousedown', handleClickOutside);
});
</script>

<template>
  <div ref="rootRef" class="relative space-y-1">
    <label v-if="props.label" class="block text-vrc-subtext text-xs">
      {{ props.label }}
    </label>
    <button
        type="button"
        :class="triggerClasses"
        :disabled="props.disabled"
        @click="toggleOpen"
        @keydown="handleKeydown"
    >
      <span class="flex-1 min-w-0 truncate">
        {{ selectedOption?.label ?? props.modelValue }}
      </span>
      <ChevronDownIcon class="text-vrc-text/50" :size="16" />
    </button>

    <div v-if="isOpen" :class="menuClasses">
      <button
          v-for="option in props.options"
          :key="option.value"
          type="button"
          class="flex gap-2 items-center px-3 py-2 text-left text-sm text-vrc-text w-full hover:bg-vrc-highlight/10"
          :class="option.value === props.modelValue ? 'bg-vrc-highlight/10' : ''"
          @click="selectOption(option.value)"
      >
        <span class="font-semibold truncate">{{ option.label }}</span>
      </button>
    </div>

    <p v-if="props.helper" class="text-[10px] text-vrc-text/60">{{ props.helper }}</p>
  </div>
</template>
