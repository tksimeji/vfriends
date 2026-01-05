import {computed, ref} from 'vue';
import {t} from '../i18n';
import {fetchFriends} from '../invokes.ts';
import {VRChat} from '../vrchat.ts';
import {isOffline} from './useFriendStatus';

const AUTO_REFRESH_MS = 30000;

const entries = ref<VRChat.LimitedUserFriend[]>([]);
const isLoading = ref(false);
const errorMessage = ref('');
const refreshTimer = ref<number | null>(null);

const STATUS_PRIORITY: Record<VRChat.UserStatus, number> = {
  'join me': 0,
  'active': 1,
  'ask me': 2,
  'busy': 3,
  'offline': 4,
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
  return first.displayName
    .toLowerCase()
    .localeCompare(second.displayName.toLowerCase());
};

const sortFriends = (friends: VRChat.LimitedUserFriend[]) =>
  [...friends].sort(compareFriends);

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

const sortedItems = computed(() => sortFriends(entries.value));
const hasFriends = computed(() => sortedItems.value.length > 0);

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
    errorMessage.value = t('friends.errors.fetchFailed');
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

export const useFriends = () => ({
  isLoading,
  errorMessage,
  sortedItems,
  hasFriends,
  refresh,
  startAutoRefresh,
  stopAutoRefresh,
});
