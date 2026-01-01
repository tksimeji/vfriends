import {computed, ref} from 'vue';
import {
  fetchNotificationPreferences,
  setNotificationPreference,
} from '../../data/notifications';
import type {
  FriendNotificationPreference,
  FriendNotificationPreferencePatch,
  NotificationPreferences,
} from '../../domain/notifications';

const preferences = ref<NotificationPreferences>({});
const isLoaded = ref(false);
const errorMessage = ref('');
let loadPromise: Promise<void> | null = null;

const load = async () => {
  if (isLoaded.value) return;
  if (loadPromise) return loadPromise;
  loadPromise = (async () => {
    try {
      const result = await fetchNotificationPreferences();
      preferences.value = result ?? {};
      errorMessage.value = '';
    } catch (error) {
      console.error(error);
      errorMessage.value = '通知設定の読み込みに失敗しました。';
    } finally {
      isLoaded.value = true;
      loadPromise = null;
    }
  })();
  return loadPromise;
};

const preferenceFor = (friendId: string) => preferences.value[friendId];

const isEnabled = (friendId: string) =>
  preferences.value[friendId]?.enabled !== false;

const normalizePatch = (patch: FriendNotificationPreferencePatch) => {
  const normalized: FriendNotificationPreferencePatch = {...patch};
  if (Object.prototype.hasOwnProperty.call(patch, 'messageTemplate')) {
    normalized.messageTemplate = patch.messageTemplate?.trim()
      ? patch.messageTemplate
      : null;
  }
  if (Object.prototype.hasOwnProperty.call(patch, 'sound')) {
    normalized.sound = patch.sound?.trim() ? patch.sound : null;
  }
  return normalized;
};

const buildOptimistic = (
  previous: FriendNotificationPreference | undefined,
  patch: FriendNotificationPreferencePatch,
) => ({
  enabled: previous?.enabled ?? true,
  messageTemplate: previous?.messageTemplate ?? null,
  sound: previous?.sound ?? null,
  ...patch,
});

const setPreference = async (
  friendId: string,
  patch: FriendNotificationPreferencePatch,
) => {
  const previous = preferences.value[friendId];
  const normalized = normalizePatch(patch);
  const next = buildOptimistic(previous, normalized);

  preferences.value = {
    ...preferences.value,
    [friendId]: next,
  };

  try {
    const payload: FriendNotificationPreferencePatch = {};
    if (Object.prototype.hasOwnProperty.call(patch, 'enabled')) {
      payload.enabled = normalized.enabled;
    }
    if (Object.prototype.hasOwnProperty.call(patch, 'messageTemplate')) {
      payload.messageTemplate = normalized.messageTemplate ?? '';
    }
    if (Object.prototype.hasOwnProperty.call(patch, 'sound')) {
      payload.sound = normalized.sound ?? '';
    }

    await setNotificationPreference(friendId, payload);
  } catch (error) {
    console.error(error);
    preferences.value = {
      ...preferences.value,
      [friendId]: previous ?? {
        enabled: true,
        messageTemplate: null,
        sound: null,
      },
    };
  }
};

const setEnabled = async (friendId: string, enabled: boolean) =>
  setPreference(friendId, {enabled});

const enabledCount = computed(
  () =>
    Object.values(preferences.value).filter((value) => value?.enabled !== false)
      .length,
);

export const useNotificationPreferences = () => {
  if (!isLoaded.value && !loadPromise) {
    void load();
  }

  return {
    isLoaded,
    errorMessage,
    preferences,
    load,
    preferenceFor,
    isEnabled,
    setEnabled,
    setPreference,
    enabledCount,
  };
};
