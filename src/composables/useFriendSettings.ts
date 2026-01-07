import {computed, onMounted, ref, type Ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {fetchFriendSettings, setFriendSettings} from '../invokes';
import {FriendSettings} from '../types.ts';
import {resolveSoundPath} from '../utils.ts';
import {useAppSettings} from './useAppSettings';

const toErrorMessage = (error: unknown, fallback: string) => {
  if (typeof error === 'string') return error;
  if (error instanceof Error) return error.message || fallback;
  return fallback;
};

export const useFriendSettings = (
  {
    friendId,
    onUpdated,
  }: {
    friendId: Ref<string>;
    onUpdated?: () => void;
  }
) => {
  const {t} = useI18n();
  const {appSettings} = useAppSettings();
  const settings = ref<FriendSettings | null>(null);
  const messageDraft = ref('');
  const soundDraft = ref('');
  const soundError = ref('');
  const isSoundSaving = ref(false);
  let soundErrorTimer: ReturnType<typeof setTimeout> | null = null;

  const loadSettings = async () => {
    const map = await fetchFriendSettings();
    settings.value = map?.[friendId.value] ?? null;
    appSettings.value.friendSettings = map ?? {};
  };

  onMounted(() => {
    void loadSettings();
  });

  watch(
    () => friendId.value,
    () => {
      void loadSettings();
    },
  );

  watch(
    () => settings.value,
    (next) => {
      messageDraft.value = next?.messageOverride ?? '';
      soundDraft.value = next?.soundOverride ?? '';
    },
    {immediate: true},
  );

  const notificationsEnabled = computed(() => settings.value?.enabled !== false);
  const customizeEnabled = computed(() => settings.value?.useOverride ?? false);
  const canCustomize = computed(
    () => notificationsEnabled.value && customizeEnabled.value,
  );

  const messageOverridden = computed(() => {
    if (!customizeEnabled.value) return false;
    return settings.value?.messageOverride !== undefined &&
      settings.value?.messageOverride !== null;
  });

  const soundOverridden = computed(() => {
    if (!customizeEnabled.value) return false;
    return settings.value?.soundOverride !== undefined &&
      settings.value?.soundOverride !== null;
  });

  const currentLabel = (overridden: boolean) =>
    t('settings.friend.currentLabel', {
      value: t(
        overridden
          ? 'settings.friend.currentCustom'
          : 'settings.friend.currentGlobal',
      ),
    });

  const currentMessageLabel = computed(() =>
    currentLabel(messageOverridden.value),
  );
  const currentSoundLabel = computed(() =>
    currentLabel(soundOverridden.value),
  );

  const setSoundError = (message: string) => {
    soundError.value = message;
    if (soundErrorTimer) clearTimeout(soundErrorTimer);
    if (!message) return;
    soundErrorTimer = setTimeout(() => {
      soundError.value = '';
    }, 3000);
  };

  const clearSoundError = () => {
    setSoundError('');
  };

  const patchSettings = async (patch: Partial<FriendSettings>) => {
    try {
      await setFriendSettings(friendId.value, patch);
      await loadSettings();
      onUpdated?.();
    } catch (error) {
      setSoundError(toErrorMessage(error, t('settings.notifications.errors.soundFailed')));
      throw error;
    }
  };

  const toggleNotifications = (value: boolean) => {
    void patchSettings({enabled: value});
  };

  const toggleCustomize = (value: boolean) => {
    void patchSettings({useOverride: value});
  };

  const handleMessageInput = (event: Event) => {
    messageDraft.value = (event.target as HTMLInputElement).value;
  };

  const commitMessage = () => {
    const trimmed = messageDraft.value.trim();
    void patchSettings({messageOverride: trimmed});
  };

  const commitPending = () => {
    const current = settings.value?.messageOverride ?? '';
    const trimmed = messageDraft.value.trim();
    if (trimmed === current) return;
    void patchSettings({messageOverride: trimmed});
  };

  const selectSound = async (file: File | null) => {
    setSoundError('');
    isSoundSaving.value = true;
    try {
      const path = await resolveSoundPath(file);
      if (!path) return;
      soundDraft.value = path;
      await patchSettings({soundOverride: path});
    } catch (error) {
      setSoundError(toErrorMessage(error, t('settings.notifications.errors.soundFailed')));
    } finally {
      isSoundSaving.value = false;
    }
  };

  const clearSound = () => {
    setSoundError('');
    soundDraft.value = '';
    void patchSettings({soundOverride: ''});
  };

  return {
    settings,
    messageDraft,
    soundDraft,
    soundError,
    isSoundSaving,
    notificationsEnabled,
    customizeEnabled,
    canCustomize,
    currentMessageLabel,
    currentSoundLabel,
    handleMessageInput,
    commitMessage,
    commitPending,
    selectSound,
    clearSound,
    toggleNotifications,
    toggleCustomize,
    clearSoundError,
  };
};
