<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {useAuthSession} from '../../composables/useAuthSession';
import {useFriends} from '../../composables/useFriends';
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
  (e: 'hover-color', rgb: [number, number, number] | null): void;
  (e: 'logout'): void;
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
  focusSidebarSearch: () => void;
};

const settingsModalRef = ref<SettingsModalHandle | null>(null);
const searchActive = computed(() => props.searchQuery.trim().length > 0);
const filteredCount = computed(() => filteredFriends.value.length);
const {t} = useI18n();
const {isAuthenticated} = useAuthSession();
const isAuthed = computed(() => Boolean(isAuthenticated.value));
const statusMessage = computed<FriendsStatusMessage | null>(() => {
  if (errorMessage.value) {
    return {
      text: errorMessage.value,
      tone: 'error',
    };
  }
  if (isLoading.value && !hasFriends.value) {
    return {
      text: t('friends.loading'),
      tone: 'muted',
    };
  }
  if (!hasFriends.value) {
    return {
      text: t('friends.empty'),
      tone: 'muted',
    };
  }
  if (searchActive.value && filteredCount.value === 0) {
    return {
      text: t('friends.searchEmpty'),
      tone: 'muted',
    };
  }
  return null;
});

const showList = computed(() => !statusMessage.value);

onMounted(() => {
  if (isAuthed.value) {
    void refresh();
    startAutoRefresh();
  }
});

onBeforeUnmount(() => {
  stopAutoRefresh();
});

watch(
  isAuthed,
  (next) => {
    if (next) {
      void refresh();
      startAutoRefresh();
    } else {
      stopAutoRefresh();
    }
  },
  {immediate: false},
);

const openSettings = () => {
  settingsModalRef.value?.openGlobal();
};

const openSettingsForFriend = (friendId: string) => {
  settingsModalRef.value?.openFriend(friendId);
};

const closeSettings = () => {
  settingsModalRef.value?.close();
};

const focusSettingsSearch = () => {
  settingsModalRef.value?.focusSidebarSearch();
};

defineExpose({
  openSettings,
  openSettingsForFriend,
  closeSettings,
  focusSettingsSearch,
});
</script>

<template>
  <div class="flex flex-1 flex-col max-w-6xl min-h-0 mx-auto px-4 pt-4 relative w-full">
    <SettingsModal
        ref="settingsModalRef"
        :friends="sortedItems"
        @open="emit('settings-opened')"
        @close="emit('settings-closed')"
        @logout="emit('logout')"
    />
    <FriendsList
        v-if="showList"
        :friends="filteredFriends"
        @hover-color="(rgb) => emit('hover-color', rgb)"
        @open-settings="openSettingsForFriend"
    />
  </div>
</template>