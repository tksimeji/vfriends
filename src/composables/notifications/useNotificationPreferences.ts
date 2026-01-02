import {computed, ref} from 'vue';
import {
  fetchNotificationPreferences,
  setNotificationPreference,
} from '../../data/notifications';
import type {
  FriendNotification,
  FriendNotificationMap,
  FriendNotificationPatch,
} from '../../domain/notifications';
import {t} from '../../i18n';

const preferences = ref<FriendNotificationMap>({});
const isLoaded = ref(false);
const errorMessage = ref('');
let loadPromise: Promise<void> | null = null;

const hasOwn = <T extends object>(
  target: T,
  key: PropertyKey,
) => Object.prototype.hasOwnProperty.call(target, key);

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
      errorMessage.value = t('notifications.errors.loadFailed');
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

const normalizeOptional = (value: string | null | undefined) => {
  if (value === null) return null;
  return value?.trim() ?? '';
};

const normalizePatch = (patch: FriendNotificationPatch) => {
  const normalized: FriendNotificationPatch = {...patch};
  if (hasOwn(patch, 'messageTemplate')) {
    normalized.messageTemplate = normalizeOptional(patch.messageTemplate);
  }
  if (hasOwn(patch, 'sound')) {
    normalized.sound = normalizeOptional(patch.sound);
  }
  return normalized;
};

const buildOptimistic = (
  previous: FriendNotification | undefined,
  patch: FriendNotificationPatch,
) => ({
  enabled: previous?.enabled ?? true,
  useCustom: previous?.useCustom ?? false,
  messageTemplate: previous?.messageTemplate ?? null,
  sound: previous?.sound ?? null,
  ...patch,
});

const buildPayload = (
  patch: FriendNotificationPatch,
  normalized: FriendNotificationPatch,
) => {
  const payload: FriendNotificationPatch = {};
  if (hasOwn(patch, 'enabled')) {
    payload.enabled = normalized.enabled;
  }
  if (hasOwn(patch, 'useCustom')) {
    payload.useCustom = normalized.useCustom;
  }
  if (hasOwn(patch, 'messageTemplate')) {
    payload.messageTemplate = normalized.messageTemplate ?? null;
  }
  if (hasOwn(patch, 'sound')) {
    payload.sound = normalized.sound ?? null;
  }
  return payload;
};

const applyPreference = (friendId: string, next: FriendNotification) => {
  preferences.value = {
    ...preferences.value,
    [friendId]: next,
  };
};

const setPreference = async (friendId: string, patch: FriendNotificationPatch) => {
  const previous = preferences.value[friendId];
  const normalized = normalizePatch(patch);
  const next = buildOptimistic(previous, normalized);

  applyPreference(friendId, next);

  try {
    await setNotificationPreference(friendId, buildPayload(patch, normalized));
  } catch (error) {
    console.error(error);
    applyPreference(friendId, previous ?? {
      enabled: true,
      useCustom: false,
      messageTemplate: null,
      sound: null,
    });
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
