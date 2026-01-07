<script setup lang="ts">
import {SearchIcon, XIcon} from 'lucide-vue-next';
import {computed, nextTick, onBeforeUpdate, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {useFriends} from '../../composables/useFriends';
import VrcAvatar from '../VrcAvatar.vue';

const MAX_VISIBLE = 10;

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'select', friend: VRChat.LimitedUserFriend): void;
}>();

const {friends} = useFriends();
const {t} = useI18n();

const rootRef = ref<HTMLElement | null>(null);
const inputRef = ref<HTMLInputElement | null>(null);
const suggestionRefs = ref<HTMLElement[]>([]);
const activeSuggestion = ref(-1);
const hoveredSuggestion = ref(-1);
const isOpen = ref(false);
const isFocused = ref(false);

const searchQuery = computed(() => props.modelValue.trim());
const normalizedQuery = computed(() => searchQuery.value.toLowerCase());
const filteredSuggestions = computed(() => {
  const normalized = normalizedQuery.value;
  if (!normalized) return [];
  return friends.value
      .filter((friend) => friend.displayName.toLowerCase().includes(normalized))
      .slice(0, MAX_VISIBLE);
});
const hasSuggestions = computed(() => filteredSuggestions.value.length > 0);
const shouldShowSuggestions = computed(() => isOpen.value && hasSuggestions.value);

const clearInput = () => {
  isOpen.value = false;
  activeSuggestion.value = -1;
  hoveredSuggestion.value = -1;
  emit('update:modelValue', '');
};

const handleInput = (event: InputEvent) => {
  isOpen.value = true;
  emit('update:modelValue', (event.target as HTMLInputElement).value);
};

const selectSuggestion = (friend: VRChat.LimitedUserFriend) => {
  emit('select', friend);
  clearInput();
};

const setActiveSuggestion = (index: number) => {
  activeSuggestion.value = index;
};

const moveActiveSuggestion = (delta: number) => {
  const count = filteredSuggestions.value.length;
  if (count === 0) return;
  const next = (activeSuggestion.value + delta + count) % count;
  setActiveSuggestion(next);
};

const selectActiveSuggestion = () => {
  const target = filteredSuggestions.value[activeSuggestion.value];
  if (!target) return;
  selectSuggestion(target);
};

const handleKeydown = (event: KeyboardEvent) => {
  switch (event.key) {
    case 'ArrowDown': {
      event.preventDefault();
      moveActiveSuggestion(1);
      return;
    }
    case 'ArrowUp': {
      event.preventDefault();
      moveActiveSuggestion(-1);
      return;
    }
    case 'Enter': {
      event.preventDefault();
      selectActiveSuggestion();
      return;
    }
    case 'Escape': {
      clearInput();
      return;
    }
  }
};

const handleFocusIn = () => {
  isFocused.value = true;
  if (searchQuery.value.length > 0) {
    isOpen.value = true;
  }
};

const handleFocusOut = () => {
  requestAnimationFrame(() => {
    const root = rootRef.value;
    if (!root) {
      isOpen.value = false;
      isFocused.value = false;
      return;
    }
    if (!root.contains(document.activeElement)) {
      isOpen.value = false;
      isFocused.value = false;
    }
  });
};

const focusInput = async () => {
  await nextTick();
  inputRef.value?.focus();
};

defineExpose({
  focus: focusInput,
});

onBeforeUpdate(() => {
  suggestionRefs.value = [];
});

watch(filteredSuggestions, (next) => {
  if (next.length === 0) {
    setActiveSuggestion(-1);
  } else if (activeSuggestion.value >= next.length) {
    setActiveSuggestion(next.length - 1);
  }
});

watch(searchQuery, (next) => {
  if (!next) {
    isOpen.value = false;
  }
});

watch(activeSuggestion, (next) => {
  if (next < 0) return;
  const el = suggestionRefs.value[next];
  if (el) {
    el.scrollIntoView({block: 'nearest'});
  }
});

onMounted(() => {
  void focusInput();
});
</script>

<template>
  <div
      ref="rootRef"
      class="max-w-md relative w-full"
      :data-tauri-drag-region="true"
      @focusin="handleFocusIn"
      @focusout="handleFocusOut"
  >
    <div
        class="bg-vrc-background-secondary flex gap-2 items-center min-w-0 px-2 relative rounded-md w-full"
        data-tauri-drag-region="false"
    >
      <input
          ref="inputRef"
          type="text"
          class="grow outline-none py-2 text-vrc-text text-xs"
          :data-tauri-drag-region="false"
          :value="modelValue"
          :placeholder="t('titleBar.search')"
          @input="handleInput"
          @keydown="handleKeydown"
      />
      <button
          v-if="searchQuery"
          type="button"
          class="p-1 rounded-md text-vrc-text/50 hover:bg-vrc-text/20 hover:text-vrc-text"
          :data-tauri-drag-region="false"
          @click="clearInput"
      >
        <XIcon :size="14"/>
      </button>
      <button
          type="button"
          class="p-1 rounded-md text-vrc-text/50 hover:bg-vrc-text/20 hover:text-vrc-text"
      >
        <SearchIcon class="text-vrc-text/50" :size="14"/>
      </button>
      <div
          class="absolute border-b-2 border-transparent bottom-0 inset-x-0 transition-colors"
          :class="isFocused ? 'border-b-vrc-highlight/70' : ''"
      />
    </div>

    <Transition name="suggestions-slide">
      <div
          v-if="shouldShowSuggestions"
          class="absolute backdrop-blur-sm bg-vrc-background/60 border border-vrc-text/20 flex flex-col gap-1 max-h-64 mt-2 overflow-y-auto p-2 rounded-md shadow-xl w-full z-50"
          data-tauri-drag-region="false"
      >
        <button
            v-for="(friend, index) in filteredSuggestions"
            ref="suggestionRefs"
            type="button"
            class="flex gap-2 items-center pl-2 pr-4 py-3 rounded-lg select-none text-left text-sm text-vrc-text w-full hover:bg-vrc-text/20"
            :key="friend.id"
            :class="index === activeSuggestion ? 'bg-vrc-text/15' : ''"
            @click="selectSuggestion(friend)"
            @mouseenter="hoveredSuggestion = index"
            @mouseleave="hoveredSuggestion = -1"
        >
          <span
              v-if="index == activeSuggestion || index == hoveredSuggestion"
              class="border-vrc-highlight border-2 h-8 rounded-4xl"
          />
          <span
              v-else
              class="border-2 border-transparent h-8"
          />
          <VrcAvatar :user="friend" class="rounded-sm"/>
          <span class="font-semibold truncate">{{ friend.displayName }}</span>
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.suggestions-slide-enter-active,
.suggestions-slide-leave-active {
  transition: opacity 0.18s ease, transform 0.18s ease;
}

.suggestions-slide-enter-from,
.suggestions-slide-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}

.suggestions-slide-enter-to,
.suggestions-slide-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

</style>
