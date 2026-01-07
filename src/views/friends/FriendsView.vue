<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {useAuthSession} from '../../composables/useAuthSession';
import {useFriends} from '../../composables/useFriends';
import {useFriendSelection} from '../../composables/useFriendSelection';
import {useFriendSelectionDrag} from '../../composables/useFriendSelectionDrag';
import {isOffline} from '../../composables/useFriendStatus';
import {setFriendSettings} from '../../invokes';
import type {VRChat} from '../../vrchat.ts';
import SettingsModal from '../settings/SettingsModal.vue';
import FriendsList from './FriendsList.vue';
import FriendsSelectionActions from './FriendsSelectionActions.vue';
import type {FriendsStatusMessage} from './types';

type SettingsModalHandle = {
  openGlobal: () => void;
  openFriend: (friend: VRChat.LimitedUserFriend) => void;
  close: () => void;
  focusSidebarSearch: () => void;
};

type FriendsListHandle = {
  getSelectoContainer: () => HTMLElement | null;
  getScrollContainer: () => HTMLElement | null;
};

const STATUS_PRIORITY: Record<VRChat.UserStatus, number> = {
  'join me': 0,
  'active': 1,
  'ask me': 2,
  'busy': 3,
  'offline': 4,
};
const HIDDEN_LOCATIONS = new Set(['offline', 'private', 'traveling', 'web']);
const settingsModalRef = ref<SettingsModalHandle | null>(null);
const friendsListRef = ref<FriendsListHandle | null>(null);
const viewportRef = ref<HTMLElement | null>(null);
const settingsVersion = ref(0);
const actionToast = ref<{ message: string; id: number } | null>(null);
let actionToastTimer: number | null = null;
const showList = ref(false);
const props = defineProps<{
  searchQuery: string;
}>();
const emit = defineEmits<{
  (e: 'settings-opened'): void;
  (e: 'settings-closed'): void;
  (e: 'hover-color', rgb: [number, number, number] | null): void;
  (e: 'logout'): void;
}>();
const {
  friends,
  isLoading,
  errorMessage,
  hasFriends,
  refresh,
  startAutoRefresh,
  stopAutoRefresh,
  setRefreshSuspended,
} = useFriends();
const {t} = useI18n();
const {isAuthenticated} = useAuthSession();
const {
  selectedIds,
  selectionAnchorId,
  hasSelection,
  setSelection,
  clearSelection,
  toggleSelection,
  applyRangeSelection,
  selectAll,
  pruneSelection,
  setOnChange,
} = useFriendSelection({
  getRangeList: () => filteredFriends.value,
});
const {handleViewportPointerDown, isSelecting} = useFriendSelectionDrag({
  viewportRef,
  listRef: friendsListRef,
  showList,
  getFilteredFriends: () => filteredFriends.value,
  selectedIds,
  selectionAnchorId,
  setSelection,
  clearSelection,
  selectAll,
  setOnSelectionChange: setOnChange,
});
const sortedFriends = computed(() =>
  [...friends.value].sort(compareFriends),
);
const filteredFriends = computed(() => {
  const query = props.searchQuery.trim().toLowerCase();
  if (!query) return sortedFriends.value;
  return sortedFriends.value.filter((friend) =>
    friend.displayName.toLowerCase().includes(query),
  );
});
const searchActive = computed(() => props.searchQuery.trim().length > 0);
const filteredCount = computed(() => filteredFriends.value.length);
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
const selectedFriends = computed(() =>
  sortedFriends.value.filter((friend) => selectedIds.value.has(friend.id)),
);

const hasKnownLocation = (friend: VRChat.LimitedUserFriend) => {
  const location = friend.location?.toLowerCase();
  return Boolean(location && !HIDDEN_LOCATIONS.has(location));
};

const compareFriends = (
  first: VRChat.LimitedUserFriend,
  second: VRChat.LimitedUserFriend,
) => {
  const rankA = isOffline(first)
    ? Number.POSITIVE_INFINITY
    : STATUS_PRIORITY[first.status] ?? Number.POSITIVE_INFINITY;
  const rankB = isOffline(second)
    ? Number.POSITIVE_INFINITY
    : STATUS_PRIORITY[second.status] ?? Number.POSITIVE_INFINITY;
  if (rankA !== rankB) return rankA - rankB;
  const locationA = hasKnownLocation(first);
  const locationB = hasKnownLocation(second);
  if (locationA !== locationB) return locationA ? -1 : 1;
  return first.displayName
    .toLowerCase()
    .localeCompare(second.displayName.toLowerCase());
};

const openSettings = () => {
  settingsModalRef.value?.openGlobal();
};

const openSettingsForFriend = (friend: VRChat.LimitedUserFriend) => {
  settingsModalRef.value?.openFriend(friend);
};

const closeSettings = () => {
  settingsModalRef.value?.close();
};

const focusSettingsSearch = () => {
  settingsModalRef.value?.focusSidebarSearch();
};

const handleFriendClick = (payload: { friend: VRChat.LimitedUserFriend; event: MouseEvent }) => {
  if (isSelecting.value) return;
  const {friend, event} = payload;
  if (!event.shiftKey && !event.ctrlKey && !event.metaKey && hasSelection.value) {
    clearSelection();
    return;
  }
  if (event.shiftKey) {
    if (event.ctrlKey || event.metaKey) {
      applyRangeSelection(friend.id, 'add');
    } else {
      applyRangeSelection(friend.id, 'replace');
    }
    return;
  }
  if (event.ctrlKey || event.metaKey) {
    toggleSelection(friend.id);
    return;
  }
  clearSelection();
  openSettingsForFriend(friend);
};

const applySelectionNotifications = async (enabled: boolean) => {
  const targetIds = [...selectedIds.value];
  if (targetIds.length === 0) return;
  try {
    await Promise.all(
      targetIds.map((friendId) => setFriendSettings(friendId, {enabled})),
    );
    settingsVersion.value += 1;
    const messageKey = enabled
      ? 'friends.bulkToast.enabled'
      : 'friends.bulkToast.disabled';
    actionToast.value = {
      message: t(messageKey, {count: targetIds.length}),
      id: Date.now(),
    };
    if (actionToastTimer) {
      window.clearTimeout(actionToastTimer);
      actionToastTimer = null;
    }
    actionToastTimer = window.setTimeout(() => {
      actionToast.value = null;
      actionToastTimer = null;
    }, 2000);
    clearSelection();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  if (isAuthed.value) {
    void refresh();
    startAutoRefresh();
  }
});

onBeforeUnmount(() => {
  stopAutoRefresh();
  if (actionToastTimer) {
    window.clearTimeout(actionToastTimer);
    actionToastTimer = null;
  }
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

watch(
  isSelecting,
  (next) => {
    setRefreshSuspended(next);
  },
);

watch(
  statusMessage,
  (next) => {
    showList.value = !next;
  },
  {immediate: true},
);

watch(
  () => friends.value,
  (next) => {
    pruneSelection(next);
  },
);

defineExpose({
  openSettings,
  openSettingsForFriend,
  closeSettings,
  focusSettingsSearch,
});
</script>

<template>
  <div
      ref="viewportRef"
      class="flex flex-1 flex-col min-h-0 relative w-full"
      @pointerdown="handleViewportPointerDown"
  >
    <div class="flex flex-1 flex-col min-h-0 pt-4 w-full">
      <SettingsModal
          ref="settingsModalRef"
          :friends="friends"
          @open="emit('settings-opened')"
          @close="emit('settings-closed')"
          @logout="emit('logout')"
      />
      <FriendsSelectionActions
          :has-selection="hasSelection"
          :selected-count="selectedIds.size"
          :selected-friends="selectedFriends"
          @clear="clearSelection"
          @disable="() => applySelectionNotifications(false)"
          @enable="() => applySelectionNotifications(true)"
      />
      <Transition name="action-toast">
        <div v-if="actionToast" class="absolute bottom-6 flex inset-x-0 justify-center px-4 z-20">
          <div
              class="backdrop-blur-md bg-black/80 border border-white/10 px-4 py-2 rounded-full shadow-[0_12px_30px_rgba(0,0,0,0.4)] text-sm text-vrc-text/80">
            {{ actionToast.message }}
          </div>
        </div>
      </Transition>
      <FriendsList
          v-if="showList"
          ref="friendsListRef"
          :friends="filteredFriends"
          :selected-ids="selectedIds"
          :settings-version="settingsVersion"
          @friend-click="handleFriendClick"
          @hover-color="(rgb) => emit('hover-color', rgb)"
      />
    </div>
  </div>
</template>

<style scoped>
.action-toast-enter-active,
.action-toast-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.action-toast-enter-from,
.action-toast-leave-to {
  opacity: 0;
  transform: translateY(8px) scale(0.98);
}

.action-toast-enter-to,
.action-toast-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}
</style>