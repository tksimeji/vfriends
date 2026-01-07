import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, ref} from 'vue';
import {t} from '../i18n';
import {fetchFriends} from '../invokes.ts';
import {VRChat} from '../vrchat.ts';

const entries = ref<VRChat.LimitedUserFriend[]>([]);
const isLoading = ref(false);
const errorMessage = ref('');
const friendEventUnlisten = ref<UnlistenFn | null>(null);
const isListening = ref(false);
let pendingSnapshot: VRChat.LimitedUserFriend[] | null = null;
let refreshFrame: number | null = null;

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

const friends = computed(() => entries.value);
const hasFriends = computed(() => entries.value.length > 0);

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

const applyFriendsSnapshot = (friends: VRChat.LimitedUserFriend[]) => {
  entries.value = mergeFriends(entries.value, friends);
};

const scheduleSnapshot = (friends: VRChat.LimitedUserFriend[]) => {
  pendingSnapshot = friends;
  if (refreshFrame !== null) return;
  if (typeof window === 'undefined' || !window.requestAnimationFrame) {
    applyFriendsSnapshot(friends);
    pendingSnapshot = null;
    return;
  }
  refreshFrame = window.requestAnimationFrame(() => {
    if (pendingSnapshot) applyFriendsSnapshot(pendingSnapshot);
    pendingSnapshot = null;
    refreshFrame = null;
  });
};

const startAutoRefresh = () => {
  if (isListening.value) return;
  isListening.value = true;
  void listen<VRChat.LimitedUserFriend[]>('vrc:friends-refresh', (event) => {
    scheduleSnapshot(event.payload);
  }).then((unlisten) => {
    if (!isListening.value) {
      unlisten();
      return;
    }
    friendEventUnlisten.value = unlisten;
  });
};

const stopAutoRefresh = () => {
  isListening.value = false;
  if (refreshFrame !== null && typeof window !== 'undefined') {
    window.cancelAnimationFrame(refreshFrame);
    refreshFrame = null;
  }
  pendingSnapshot = null;
  if (friendEventUnlisten.value === null) return;
  friendEventUnlisten.value();
  friendEventUnlisten.value = null;
};

export const useFriends = () => ({
  isLoading,
  errorMessage,
  friends,
  hasFriends,
  refresh,
  startAutoRefresh,
  stopAutoRefresh,
});
