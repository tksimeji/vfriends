import {invoke} from '@tauri-apps/api/core';
import {computed, ref} from 'vue';
import {VRChat} from '../vrchat.ts';

const AUTO_REFRESH_MS = 30000;
const STATUS_PRIORITY: Record<VRChat.UserStatus, number> = {
  'join me': 0,
  'active': 1,
  'ask me': 2,
  'busy': 3,
  'offline': 4,
};

const mergeFriends = (
  current: VRChat.LimitedUserFriend[],
  next: VRChat.LimitedUserFriend[],
) => {
  const byId = new Map(current.map((friend) => [friend.id, friend]));
  return next.map((friend) => {
    const existing = byId.get(friend.id);
    if (!existing) return friend;
    Object.assign(existing, friend);
    return existing;
  });
};

const rankFriend = (friend: VRChat.LimitedUserFriend) => {
  if (VRChat.isOffline(friend)) return Number.POSITIVE_INFINITY;
  const statusKey = friend.status;
  return STATUS_PRIORITY[statusKey] ?? Number.POSITIVE_INFINITY;
};

const compareFriends = (a: VRChat.LimitedUserFriend, b: VRChat.LimitedUserFriend) => {
  const rankA = rankFriend(a);
  const rankB = rankFriend(b);
  if (rankA !== rankB) return rankA - rankB;
  return a.displayName
    .toLowerCase()
    .localeCompare(b.displayName.toLowerCase());
};

export const useFriends = () => {
  const entries = ref<VRChat.LimitedUserFriend[]>([]);
  const isLoading = ref(false);
  const errorMessage = ref('');
  const refreshTimer = ref<number | null>(null);

  const sortedItems = computed(() => [...entries.value].sort(compareFriends));

  const hasFriends = computed(() => sortedItems.value.length > 0);

  const fetchFriends = async () =>
    invoke<VRChat.LimitedUserFriend[]>('fetch_friends');

  const refresh = async () => {
    if (isLoading.value) return;
    isLoading.value = true;
    errorMessage.value = '';
    try {
      const result = await fetchFriends();
      const nextEntries = result ?? [];
      entries.value = mergeFriends(entries.value, nextEntries);
    } catch (error) {
      console.error(error);
      errorMessage.value = 'フレンド一覧の取得に失敗しました。';
    } finally {
      isLoading.value = false;
    }
  };

  const startAutoRefresh = (intervalMs = AUTO_REFRESH_MS) => {
    if (refreshTimer.value !== null) return;
    refreshTimer.value = window.setInterval(() => {
      void refresh();
    }, intervalMs);
  };

  const stopAutoRefresh = () => {
    if (refreshTimer.value === null) return;
    window.clearInterval(refreshTimer.value);
    refreshTimer.value = null;
  };

  return {
    isLoading,
    errorMessage,
    sortedItems,
    hasFriends,
    refresh,
    startAutoRefresh,
    stopAutoRefresh,
  };
};
