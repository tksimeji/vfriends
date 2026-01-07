<script setup lang="ts">
import {Listbox, ListboxButton, ListboxOption, ListboxOptions} from '@headlessui/vue';
import {CheckIcon, ChevronDownIcon} from 'lucide-vue-next';
import {computed} from 'vue';

type Option = {
  value: string;
  label: string;
};

const props = withDefaults(defineProps<{
  modelValue: string;
  label?: string;
  disabled?: boolean;
  options: Option[];
}>(), {
  label: '',
  disabled: false,
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const selectedValue = computed({
  get: () => props.modelValue,
  set: (value: string) => emit('update:modelValue', value),
});
const selectedOption = computed(() => props.options.find((option) => option.value === props.modelValue) ?? null);
</script>

<template>
  <div class="relative space-y-1">
    <label v-if="props.label" class="block font-semibold text-vrc-text text-md">
      {{ props.label }}
    </label>

    <Listbox v-model="selectedValue" :disabled="props.disabled" v-slot="{ open }">
      <div class="relative">
        <ListboxButton
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex items-center justify-between px-3 py-2 rounded-md text-left text-sm text-vrc-text transition w-full
             disabled:cursor-not-allowed disabled:opacity-50
             focus-visible:border-vrc-highlight/70 focus-visible:ring-2 focus-visible:ring-vrc-highlight/40
             hover:border-vrc-highlight/60"
        >
          <span class="flex-1 min-w-0 truncate">
            {{ selectedOption?.label ?? props.modelValue }}
          </span>
          <Transition name="chevron-bounce" mode="out-in">
            <span
                :key="open ? 'open' : 'closed'"
                class="text-vrc-text/50"
                :data-open="open"
            >
              <ChevronDownIcon :size="16"/>
            </span>
          </Transition>
        </ListboxButton>

        <Transition name="vrc-select-options">
          <ListboxOptions
              class="absolute backdrop-blur-md bg-vrc-background-secondary/80 border border-vrc-highlight/20 left-0 max-h-60 mt-2 overflow-y-auto rounded-md shadow-xl w-full z-50"
          >
            <ListboxOption
                v-for="option in props.options"
                class="cursor-pointer"
                :key="option.value"
                :value="option.value"
            >
              <template #default="{ active, selected }">
                <div
                    class="flex gap-2 items-center px-3 py-2 text-left text-sm text-vrc-text w-full hover:bg-vrc-highlight/30"
                    :class="active ? 'bg-vrc-highlight/10' : ''"
                >
                  <span class="flex-1 font-semibold min-w-0 truncate">{{ option.label }}</span>
                  <CheckIcon v-if="selected" class="text-vrc-highlight" :size="16"/>
                </div>
              </template>
            </ListboxOption>
          </ListboxOptions>
        </Transition>
      </div>
    </Listbox>
  </div>
</template>

<style scoped>
.chevron-bounce-enter-active[data-open='true'] {
  animation: chevron-bounce 0.18s ease;
}

.chevron-bounce-leave-active {
  animation: none;
}

.vrc-select-options-enter-active,
.vrc-select-options-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.vrc-select-options-enter-from,
.vrc-select-options-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.95);
}

.vrc-select-options-enter-to,
.vrc-select-options-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

@keyframes chevron-bounce {
  0% {
    opacity: 1;
    transform: translateY(0);
  }
  60% {
    opacity: 1;
    transform: translateY(4px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>