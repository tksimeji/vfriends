<script setup lang="ts">
import {computed, ref} from 'vue';
import {FolderOpenIcon} from 'lucide-vue-next';
import {useI18n} from 'vue-i18n';

const props = withDefaults(defineProps<{
  label: string;
  value: string;
  helper?: string;
  disabled?: boolean;
  accept?: string;
  clearable?: boolean;
}>(), {
  helper: '',
  disabled: false,
  accept: '',
  clearable: false,
});

const emit = defineEmits<{
  (e: 'select', file: File | null): void;
  (e: 'clear'): void;
}>();

const inputRef = ref<HTMLInputElement | null>(null);
const {t} = useI18n();

const triggerSelect = () => {
  if (props.disabled) return;
  inputRef.value?.click();
};

const handleChange = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0] ?? null;
  emit('select', file);
  input.value = '';
};

const buttonClasses = computed(() => {
  const base =
    'flex w-full items-center gap-2 rounded-md border-2 px-3 py-2 text-left text-sm outline-none transition focus-visible:ring-2 focus-visible:ring-vrc-highlight/40';
  const state = props.disabled
    ? 'border-vrc-highlight/10 bg-vrc-button/50 text-vrc-text/40 cursor-not-allowed'
    : 'border-vrc-highlight/20 bg-vrc-button/80 text-vrc-text hover:border-vrc-highlight/60';
  return `${base} ${state}`;
});
</script>

<template>
  <div class="space-y-1">
    <label class="block text-vrc-subtext text-xs">{{ props.label }}</label>
    <div class="flex gap-2 items-center">
      <button type="button" :class="buttonClasses" :disabled="props.disabled" @click="triggerSelect">
        <FolderOpenIcon :size="16" />
        <span class="flex-1 min-w-0 truncate">{{ props.value }}</span>
        <span class="text-[10px] text-vrc-text/60">{{ t('common.select') }}</span>
      </button>
      <button
          v-if="props.clearable"
          type="button"
          class="border-2 border-vrc-highlight/20 flex h-9 items-center justify-center rounded-md text-sm text-vrc-text/60 transition w-9 disabled:opacity-40 hover:border-vrc-highlight/60 hover:text-vrc-text"
          :disabled="props.disabled"
          :aria-label="t('common.clear')"
          @click="emit('clear')"
      >
        ×
      </button>
    </div>
    <p v-if="props.helper" class="text-[10px] text-vrc-text/60">{{ props.helper }}</p>
    <input
        ref="inputRef"
        type="file"
        class="hidden"
        :accept="props.accept"
        @change="handleChange"
    />
  </div>
</template>

