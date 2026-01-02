import {computed, ref} from 'vue';
import {fetchFriends} from '../data/friends';
import {VRChat} from '../vrchat.ts';

const AUTO_REFRESH_MS = 30000;
export const useFriends = () => {
  const entries = ref<VRChat.LimitedUserFriend[]>([]);
  const isLoading = ref(false);
  const errorMessage = ref('');
  const refreshTimer = ref<number | null>(null);

  const sortedItems = computed(() => VRChat.sortFriends(entries.value));

  const hasFriends = computed(() => sortedItems.value.length > 0);

  const refresh = async () => {
    if (isLoading.value) return;
    isLoading.value = true;
    errorMessage.value = '';
    try {
      const result = await fetchFriends();
      const nextEntries = result ?? [];
      entries.value = VRChat.mergeFriends(entries.value, nextEntries);
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
