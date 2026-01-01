<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {useFriends} from '../../composables/useFriends';
import type {VRChat} from '../../vrchat.ts';
import SettingsModal from '../settings/SettingsModal.vue';
import FriendsList from './FriendsList.vue';
import type {FriendsStatusMessage} from './types';

const {
  sortedItems,
  isLoading,
  errorMessage,
  hasFriends,
  refresh,
  startAutoRefresh,
  stopAutoRefresh,
} = useFriends();

const props = defineProps<{
  searchQuery: string;
}>();

const emit = defineEmits<{
  (e: 'settings-opened'): void;
  (e: 'settings-closed'): void;
  (e: 'suggestions-updated', suggestions: VRChat.LimitedUserFriend[]): void;
}>();

const filteredFriends = computed(() => {
  const query = props.searchQuery.trim().toLowerCase();
  if (!query) return sortedItems.value;
  return sortedItems.value.filter((friend) =>
    friend.displayName.toLowerCase().includes(query),
  );
});

type SettingsModalHandle = {
  openGlobal: () => void;
  openFriend: (friendId: string) => void;
  close: () => void;
};

const settingsModalRef = ref<SettingsModalHandle | null>(null);
const searchActive = computed(() => props.searchQuery.trim().length > 0);
const totalCount = computed(() => sortedItems.value.length);
const filteredCount = computed(() => filteredFriends.value.length);
const statusMessage = computed<FriendsStatusMessage | null>(() => {
  if (errorMessage.value) {
    return {
      text: errorMessage.value,
      tone: 'error',
    };
  }
  if (isLoading.value && !hasFriends.value) {
    return {
      text: 'フレンド一覧を読み込み中...',
      tone: 'muted',
    };
  }
  if (!hasFriends.value) {
    return {
      text: 'フレンドが見つかりません。',
      tone: 'muted',
    };
  }
  if (searchActive.value && filteredCount.value === 0) {
    return {
      text: '検索結果が見つかりません。',
      tone: 'muted',
    };
  }
  return null;
});

const showList = computed(() => !statusMessage.value);

watch(
  sortedItems,
  () => {
    emit('suggestions-updated', sortedItems.value);
  },
  {immediate: true},
);

onMounted(() => {
  void refresh();
  startAutoRefresh();
});

onBeforeUnmount(() => {
  stopAutoRefresh();
});

const openSettings = () => {
  settingsModalRef.value?.openGlobal();
};

const openSettingsForFriend = (friendId: string) => {
  settingsModalRef.value?.openFriend(friendId);
};

const closeSettings = () => {
  settingsModalRef.value?.close();
};

defineExpose({
  openSettings,
  openSettingsForFriend,
  closeSettings,
});
</script>

<template>
  <div class="flex flex-1 flex-col max-w-6xl min-h-0 mx-auto relative w-full">
    <SettingsModal
        ref="settingsModalRef"
        :friends="sortedItems"
        @open="emit('settings-opened')"
        @close="emit('settings-closed')"
    />

    <div class="border-b border-vrc-highlight/15 flex flex-wrap gap-3 items-center justify-between mb-2 py-2 text-vrc-text/60 text-xs">
      <p
          v-if="statusMessage"
          class="text-sm"
          :class="statusMessage.tone === 'error' ? 'text-red-300' : 'text-vrc-text/70'"
      >
        {{ statusMessage.text }}
      </p>
      <div class="flex gap-3 items-center">
        <span>{{ searchActive ? `${filteredCount} / ${totalCount}件` : `${totalCount}件` }}</span>
        <span class="hiddensm:inline">カードをクリックで通知設定</span>
        <span v-if="isLoading" class="text-vrc-highlight/70">更新中...</span>
      </div>
    </div>

    <FriendsList
        v-if="showList"
        :friends="filteredFriends"
        @open-settings="openSettingsForFriend"
    />
  </div>
</template>

