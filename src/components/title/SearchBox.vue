<script setup lang="ts">
import {SearchIcon, XIcon} from 'lucide-vue-next';
import {computed, nextTick, onBeforeUpdate, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {VRChat} from '../../vrchat.ts';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
  suggestions?: VRChat.LimitedUserFriend[];
  autoFocus?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'select', friendId: string): void;
}>();

const searchActive = computed(() => props.modelValue.trim().length > 0);
const normalizedQuery = computed(() => props.modelValue.trim().toLowerCase());
const activeIndex = ref(-1);
const suggestionRefs = ref<HTMLElement[]>([]);
const inputRef = ref<HTMLInputElement | null>(null);
const {t} = useI18n();

const resolvedPlaceholder = computed(
  () => props.placeholder ?? t('friends.searchPlaceholderTitlebar'),
);

const MAX_VISIBLE = 10;

const clearSearch = () => {
  emit('update:modelValue', '');
};

const filteredSuggestions = computed(() => {
  const query = normalizedQuery.value;
  if (!query) return [];
  const source = props.suggestions ?? [];
  return source
    .filter((friend) => friend.displayName.toLowerCase().includes(query))
    .slice(0, MAX_VISIBLE);
});

const hasSuggestions = computed(() => filteredSuggestions.value.length > 0);

const handleInput = (event: Event) => {
  emit('update:modelValue', (event.target as HTMLInputElement).value);
};

const selectSuggestion = (friendId: string) => {
  emit('select', friendId);
  clearSearch();
};

const moveActive = (delta: number) => {
  const count = filteredSuggestions.value.length;
  if (count === 0) return;
  const next = (activeIndex.value + delta + count) % count;
  activeIndex.value = next;
};

const handleKeydown = (event: KeyboardEvent) => {
  if (!hasSuggestions.value) return;
  if (event.key === 'ArrowDown') {
    event.preventDefault();
    moveActive(1);
    return;
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault();
    moveActive(-1);
    return;
  }
  if (event.key === 'Enter') {
    const target = filteredSuggestions.value[activeIndex.value];
    if (target) {
      event.preventDefault();
      selectSuggestion(target.id);
    }
    return;
  }
  if (event.key === 'Escape') {
    clearSearch();
  }
};

const focusInput = async () => {
  if (!props.autoFocus) return;
  await nextTick();
  inputRef.value?.focus();
};

onBeforeUpdate(() => {
  suggestionRefs.value = [];
});

watch(filteredSuggestions, (next) => {
  if (next.length === 0) {
    activeIndex.value = -1;
  } else if (activeIndex.value >= next.length) {
    activeIndex.value = next.length - 1;
  }
});

watch(activeIndex, (next) => {
  if (next < 0) return;
  const el = suggestionRefs.value[next];
  if (el) {
    el.scrollIntoView({block: 'nearest'});
  }
});

watch(
  () => props.autoFocus,
  (next) => {
    if (next) {
      void focusInput();
    }
  },
);

onMounted(() => {
  void focusInput();
});
</script>

<template>
  <div class="relative max-w-md w-full" data-tauri-drag-region>
    <div
        class="bg-vrc-background-secondary border-b border-b-transparent flex gap-2 items-center min-w-0 px-2 rounded-md w-full focus:border-b focus-within:border-b-vrc-highlight"
        data-tauri-drag-region="false"
    >
      <input
          :value="modelValue"
          ref="inputRef"
          type="text"
          class="grow outline-none py-2 text-vrc-text text-xs"
          :placeholder="resolvedPlaceholder"
          data-tauri-drag-region="false"
          @input="handleInput"
          @keydown="handleKeydown"
      />
      <button
          v-if="searchActive"
          type="button"
          class="p-1 rounded-md text-vrc-text/50 hover:bg-vrc-text/20 hover:text-vrc-text"
          data-tauri-drag-region="false"
          @click="clearSearch"
      >
        <XIcon :size="14"/>
      </button>
      <button
          type="button"
          class="p-1 rounded-md text-vrc-text/50 hover:bg-vrc-text/20 hover:text-vrc-text"
      >
        <SearchIcon class="text-vrc-text/50" :size="14"/>
      </button>
    </div>

    <div
        v-if="hasSuggestions"
        class="absolute bg-vrc-background-secondary border border-vrc-highlight/20 max-h-64 mt-2 overflow-y-auto rounded-md shadow-xl w-full z-50"
        data-tauri-drag-region="false"
    >
      <button
          v-for="(friend, index) in filteredSuggestions"
          :key="friend.id"
          ref="suggestionRefs"
          type="button"
          class="flex gap-2 items-center px-3 py-2 text-left text-sm text-vrc-text w-full hover:bg-vrc-highlight/10"
          :class="index === activeIndex ? 'bg-vrc-highlight/15' : ''"
          @click="selectSuggestion(friend.id)"
      >
        <span class="font-semibold truncate">{{ friend.displayName }}</span>
      </button>
    </div>
  </div>
</template>
