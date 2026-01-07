<script setup lang="ts">
import {open} from '@tauri-apps/plugin-dialog';
import {FolderClosedIcon, XIcon} from 'lucide-vue-next';
import {computed, ref} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from './VrcButton.vue';

const audioExtensions = ['mp3', 'wav', 'ogg', 'flac', 'm4a'];
const inputRef = ref<HTMLInputElement | null>(null);
const props = withDefaults(
    defineProps<{
      label: string;
      value: string | null;
      error?: string | null;
      disabled?: boolean;
      accept?: string;
      clearable?: boolean;
    }>(),
    {
      value: null,
      error: null,
      disabled: false,
      accept: '',
      clearable: false,
    }
);
const emit = defineEmits<{
  (e: 'select', file: File | null): void;
  (e: 'clear'): void;
  (e: 'clear-error'): void;
}>();
const {t} = useI18n();
const displayValue = computed(() => {
  const value = props.value ?? '';
  if (!value) return t('filePicker.noFileSelected');
  const parts = value.split(/[\\/]/);
  return parts[parts.length - 1] || t('filePicker.noFileSelected');
});
const displayText = computed(() => props.error?.trim() || displayValue.value);
const hasError = computed(() => Boolean(props.error?.trim()));

const buildFileFromPath = (path: string) => {
  const name = path.split(/[\\/]/).pop() ?? path;
  const file = new File([], name);
  (file as File & { path?: string }).path = path;
  return file;
};

const handleSelect = async () => {
  if (props.disabled) return;
  if (hasError.value) emit('clear-error');
  try {
    const selection = await open({
      directory: false,
      filters: [
        {
          extensions: audioExtensions,
          name: t('filePicker.audioFiles'),
        },
      ],
      multiple: false,
    });
    if (typeof selection === 'string' && selection) {
      emit('select', buildFileFromPath(selection));
      return;
    }
  } catch (error) {
    console.warn(error);
  }
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
          class="border-b-2 cursor-pointer flex gap-2 grow h-10 items-center min-w-0 outline-none px-3 rounded-md text-left text-sm transition focus-visible:ring-2 focus-visible:ring-vrc-highlight"
          :class="props.disabled ? 'bg-vrc-button/50 border-vrc-highlight/10 cursor-not-allowed text-vrc-text/40' : 'bg-vrc-background/90 border-b-vrc-highlight/40 text-vrc-text hover:border-vrc-highlight/60'"
          :disabled="props.disabled"
          @click="handleSelect"
      >
        <div class="flex flex-1 gap-2 items-center min-w-0">
          <FolderClosedIcon class="text-vrc-icon" :size="16"/>
          <span class="flex-1 min-w-0 truncate" :class="hasError ? 'text-red-200' : ''">
            {{ displayText }}
          </span>
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