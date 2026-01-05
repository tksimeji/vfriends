<script setup lang="ts">
import {FolderClosedIcon, XIcon} from 'lucide-vue-next';
import {computed, ref} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from './VrcButton.vue';

const props = withDefaults(
    defineProps<{
      label: string;
      value: string | null;
      disabled?: boolean;
      accept?: string;
      clearable?: boolean;
    }>(),
    {
      value: null,
      disabled: false,
      accept: '',
      clearable: false,
    }
);

const emit = defineEmits<{
  (e: 'select', file: File | null): void;
  (e: 'clear'): void;
}>();

const inputRef = ref<HTMLInputElement | null>(null);

const displayValue = computed(() => {
  const value = props.value ?? '';
  if (!value) return t('filePicker.noFileSelected');
  const parts = value.split(/[\\/]/);
  return parts[parts.length - 1] || t('filePicker.noFileSelected');
});

const {t} = useI18n();

const handleSelect = () => {
  if (props.disabled) return;
  inputRef.value?.click();
};

const handleChange = (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0] ?? null;
  emit('select', file);
  input.value = '';
};
</script>

<template>
  <div class="space-y-1">
    <label class="block font-semibold text-md text-vrc-text">{{ props.label }}</label>

    <div class="flex gap-4 items-center min-w-0">
      <button
          type="button"
          class="border-b-2 cursor-pointer flex gap-2 grow h-10 items-center min-w-0 outline-none px-3 rounded-md text-left text-sm transition
           focus-visible:ring-2 focus-visible:ring-vrc-highlight"
          :class="props.disabled ? 'bg-vrc-button/50 border-vrc-highlight/10 cursor-not-allowed text-vrc-text/40' : 'bg-vrc-background/90 border-b-vrc-highlight/40 text-vrc-text hover:border-vrc-highlight/60'"
          :disabled="props.disabled"
          @click="handleSelect"
      >
        <div class="flex flex-1 gap-2 items-center min-w-0">
          <FolderClosedIcon class="text-vrc-icon" :size="16"/>
          <span class="flex-1 min-w-0 truncate">{{ displayValue }}</span>
        </div>
      </button>
      <VrcButton
          v-if="props.value"
          class="h-10!"
          @click="emit('clear')"
      >
        <XIcon class="text-vrc-icon" :size="16"/>
        {{ t('filePicker.clear') }}
      </VrcButton>
    </div>
    <input
        ref="inputRef"
        type="file"
        class="hidden"
        :accept="props.accept"
        @change="handleChange"
    />
  </div>
</template>
