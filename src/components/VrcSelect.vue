<script setup lang="ts">
import {
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions,
} from '@headlessui/vue';
import {CheckIcon, ChevronDownIcon} from 'lucide-vue-next';
import {computed} from 'vue';

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

const selectedValue = computed({
  get: () => props.modelValue,
  set: (value: string) => emit('update:modelValue', value),
});

const selectedOption = computed(
  () => props.options.find((option) => option.value === props.modelValue) ?? null,
);
</script>

<template>
  <div class="relative space-y-1">
    <label v-if="props.label" class="block text-vrc-subtext text-xs">
      {{ props.label }}
    </label>

    <Listbox v-model="selectedValue" :disabled="props.disabled">
      <div class="relative">
        <ListboxButton
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex items-center justify-between px-3 py-2 rounded-md text-left text-sm text-vrc-text transition w-full disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-vrc-highlight/70 focus-visible:ring-2 focus-visible:ring-vrc-highlight/40 hover:border-vrc-highlight/60"
        >
          <span class="flex-1 min-w-0 truncate">
            {{ selectedOption?.label ?? props.modelValue }}
          </span>
          <ChevronDownIcon class="text-vrc-text/50" :size="16" />
        </ListboxButton>

        <ListboxOptions
            class="absolute bg-vrc-background-secondary border border-vrc-highlight/20 left-0 max-h-60 mt-2 overflow-y-auto rounded-md shadow-xl w-full z-50"
        >
          <ListboxOption
              v-for="option in props.options"
              :key="option.value"
              :value="option.value"
              class="cursor-pointer"
          >
            <template #default="{ active, selected }">
              <div
                  class="flex gap-2 items-center px-3 py-2 text-left text-sm text-vrc-text w-full hover:bg-vrc-highlight/10"
                  :class="active ? 'bg-vrc-highlight/10' : ''"
              >
                <span class="flex-1 font-semibold min-w-0 truncate">{{ option.label }}</span>
                <CheckIcon v-if="selected" class="text-vrc-highlight" :size="14" />
              </div>
            </template>
          </ListboxOption>
        </ListboxOptions>
      </div>
    </Listbox>

    <p v-if="props.helper" class="text-[10px] text-vrc-text/60">{{ props.helper }}</p>
  </div>
</template>
