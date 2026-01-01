<script setup lang="ts">
import {computed, onBeforeUpdate, ref, watch} from 'vue';
import {SearchIcon, XIcon} from 'lucide-vue-next';
import {VRChat} from '../vrchat.ts';
import UserAvatar from './UserAvatar.vue';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
  suggestions?: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'select', friendId: string): void;
}>();

const searchActive = computed(() => props.modelValue.trim().length > 0);
const normalizedQuery = computed(() => props.modelValue.trim().toLowerCase());
const isOpen = ref(false);
const activeIndex = ref(-1);
const suggestionRefs = ref<HTMLElement[]>([]);

const MAX_VISIBLE = 10;

const filteredSuggestions = computed(() => {
  const query = normalizedQuery.value;
  if (!query) return [];
  const list = props.suggestions ?? [];
  return list
    .filter((item) => item.displayName.toLowerCase().includes(query))
    .slice(0, MAX_VISIBLE);
});

const showSuggestions = computed(
  () => isOpen.value && filteredSuggestions.value.length > 0,
);

const clearSearch = () => {
  emit('update:modelValue', '');
};

const openSuggestions = () => {
  isOpen.value = true;
};

const closeSuggestions = () => {
  isOpen.value = false;
  activeIndex.value = -1;
};

const selectSuggestion = (
  suggestion: VRChat.LimitedUserFriend,
) => {
  emit('update:modelValue', suggestion.displayName);
  emit('select', suggestion.id);
  closeSuggestions();
};

const handleInput = (event: Event) => {
  emit('update:modelValue', (event.target as HTMLInputElement).value);
  openSuggestions();
};

const handleFocus = () => {
  openSuggestions();
};

const handleBlur = () => {
  closeSuggestions();
};

const handleKeydown = (event: KeyboardEvent) => {
  const suggestions = filteredSuggestions.value;
  if (event.key === 'ArrowDown') {
    event.preventDefault();
    if (!isOpen.value) {
      openSuggestions();
    }
    if (suggestions.length === 0) return;
    activeIndex.value =
      activeIndex.value < 0
        ? 0
        : (activeIndex.value + 1) % suggestions.length;
    return;
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault();
    if (!isOpen.value) {
      openSuggestions();
    }
    if (suggestions.length === 0) return;
    activeIndex.value =
      activeIndex.value <= 0
        ? suggestions.length - 1
        : activeIndex.value - 1;
    return;
  }
  if (event.key === 'Enter') {
    if (showSuggestions.value && activeIndex.value >= 0) {
      event.preventDefault();
      const target = suggestions[activeIndex.value];
      if (target) {
        selectSuggestion(target);
      }
    }
    return;
  }
  if (event.key === 'Escape') {
    closeSuggestions();
  }
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
  () => normalizedQuery.value,
  (value) => {
    if (!value) {
      closeSuggestions();
    }
  },
);
</script>

<template>
  <div class="max-w-md min-w-0 relative w-full" data-tauri-drag-region="false">
    <input
        :value="modelValue"
        type="text"
        class="bg-vrc-button/80 border-2 border-vrc-highlight/20 outline-none pl-3 pr-12 py-2 rounded-md text-vrc-text text-xs w-full focus:border-vrc-highlight/70"
        :placeholder="placeholder ?? 'フレンド検索...'"
        data-tauri-drag-region="false"
        @input="handleInput"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeydown"
    />
    <button
        v-if="searchActive"
        type="button"
        class="-translate-y-1/2 absolute right-8 text-vrc-text/50 top-1/2 hover:text-vrc-text"
        data-tauri-drag-region="false"
        @click="clearSearch"
    >
      <XIcon :size="14" />
    </button>
    <SearchIcon class="-translate-y-1/2 absolute pointer-events-none right-3 text-vrc-text/50 top-1/2" :size="14" />

    <div
        v-if="showSuggestions"
        class="absolute backdrop-blur bg-vrc-background-secondary/95 border-2 border-vrc-highlight/25 left-0 mt-2 p-1 right-0 rounded-md shadow-[0_18px_32px_-26px_rgba(0,0,0,0.9)] top-full z-50"
        data-tauri-drag-region="false"
        @mousedown.prevent
    >
      <ul class="max-h-64 overflow-y-auto py-1 text-vrc-text text-xs" role="listbox">
        <li v-for="(item, index) in filteredSuggestions" :key="item.id">
          <button
              type="button"
              class="flex gap-2 items-center px-3 py-2 rounded-md text-left transition w-full"
              :class="index === activeIndex ? 'bg-vrc-highlight/15 text-vrc-friend' : 'hover:bg-vrc-highlight/10'"
              data-tauri-drag-region="false"
              @mousedown.prevent
              @click="selectSuggestion(item)"
              :ref="(el) => el && suggestionRefs.push(el as HTMLElement)"
          >
            <UserAvatar
                :src="VRChat.avatarUrl(item)"
                :name="item.displayName"
                size-class="size-6"
                fallback-class="font-semibold text-[10px]"
            />
            <span class="flex flex-1 flex-col min-w-0">
              <span class="font-semibold truncate">{{ item.displayName }}</span>
              <span class="flex gap-1 items-center text-[10px] text-vrc-text/60">
                <span class="rounded-full size-2" :class="VRChat.statusColorClass(VRChat.statusKey(item))" />
                {{ VRChat.statusLabel(VRChat.statusKey(item)) }}
              </span>
            </span>
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

