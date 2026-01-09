import {computed, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {fetchAppSettings, previewNotificationSound, setAppSettings} from '../invokes';
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {AppSettings} from '../types.ts';
import {resolveSoundPath} from '../utils.ts';

const toErrorMessage = (error: unknown, fallback: string) => {
  if (typeof error === 'string') return error;
  if (error instanceof Error) return error.message || fallback;
  return fallback;
};

const createDefaults = (defaultMessage: string): AppSettings => ({
  defaultMessage,
  defaultSound: null,
  friendSettings: {},
});

const createState = () => {
  const {t} = useI18n();

  const appSettings = ref<AppSettings>(
    createDefaults(t('settings.notifications.defaultMessageTemplate')),
  );
  const errorMessage = ref('');
  const soundError = ref('');
  const isReady = ref(false);
  const isSoundSaving = ref(false);
  const isPreviewing = ref(false);
  let previewUnlisten: UnlistenFn | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let soundErrorTimer: ReturnType<typeof setTimeout> | null = null;

  const previewText = computed(() =>
    appSettings.value.defaultMessage
      .trim()
      .replace('{name}', t('settings.vrchatUserFallback'))
      .replace('{displayName}', t('settings.vrchatUserFallback')) ||
    t('settings.notifications.defaultMessageTemplate'),
  );

  const applySnapshot = (result?: AppSettings | null) => {
    appSettings.value = {
      defaultMessage: result?.defaultMessage ?? t('settings.notifications.defaultMessageTemplate'),
      defaultSound: result?.defaultSound ?? null,
      friendSettings: result?.friendSettings ?? {},
    };
  };

  const saveNow = async () => {
    try {
      const result = await setAppSettings({
        defaultMessage: appSettings.value.defaultMessage,
        defaultSound: appSettings.value.defaultSound ?? '',
      });
      applySnapshot(result);
    } catch (error) {
      console.error(error);
      errorMessage.value = t('settings.notifications.errors.saveFailed');
    }
  };

  const refresh = async () => {
    isReady.value = false;
    try {
      const result = await fetchAppSettings();
      appSettings.value = {
        defaultMessage: result?.defaultMessage ?? t('settings.notifications.defaultMessageTemplate'),
        defaultSound: result?.defaultSound ?? null,
        friendSettings: result?.friendSettings ?? {},
      };
      errorMessage.value = '';
    } catch (error) {
      console.error(error);
      errorMessage.value = t('settings.notifications.errors.loadFailed');
    } finally {
      isReady.value = true;
    }
  };

  const queueSave = () => {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => {
      void saveNow();
    }, 500);
  };

  watch(
    () => [appSettings.value.defaultMessage, appSettings.value.defaultSound],
    (next, prev) => {
      if (!isReady.value) return;
      if (next[0] === prev[0] && next[1] === prev[1]) return;
      queueSave();
    },
  );

  const setDefaultMessage = (value: string) => {
    appSettings.value.defaultMessage = value;
  };

  const setDefaultSound = (value: string | null) => {
    appSettings.value.defaultSound = value;
  };

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

  const selectDefaultSound = async (file: File | null) => {
    setSoundError('');
    isSoundSaving.value = true;
    try {
      const path = await resolveSoundPath(file);
      if (!path) return;
      appSettings.value.defaultSound = path;
    } catch (error) {
      console.error(error);
      setSoundError(toErrorMessage(error, t('settings.notifications.errors.soundFailed')));
    } finally {
      isSoundSaving.value = false;
    }
  };

  const clearDefaultSound = () => {
    setSoundError('');
    appSettings.value.defaultSound = null;
    queueSave();
  };

  let previewFallbackTimer: ReturnType<typeof setTimeout> | null = null;

  const scheduleFallback = () => {
    if (previewFallbackTimer) clearTimeout(previewFallbackTimer);
    previewFallbackTimer = setTimeout(() => {
      isPreviewing.value = false;
    }, 15_000);
  };

  const clearFallback = () => {
    if (previewFallbackTimer) {
      clearTimeout(previewFallbackTimer);
      previewFallbackTimer = null;
    }
  };

  const previewSound = async (value?: string | null) => {
    if (isPreviewing.value) return;
    isPreviewing.value = true;
    try {
      const resolved = value?.trim();
      await previewNotificationSound(resolved ? resolved : null);
      scheduleFallback();
    } catch (error) {
      console.error(error);
      isPreviewing.value = false;
    }
  };

  const previewDefaultSound = async () => {
    await previewSound(appSettings.value.defaultSound);
  };

  const flushSave = async () => {
    if (saveTimer) {
      clearTimeout(saveTimer);
      saveTimer = null;
    }
    await saveNow();
  };

  const setupPreviewListener = async () => {
    if (previewUnlisten) return;
    previewUnlisten = await listen('vrc:preview-sound-ended', () => {
      isPreviewing.value = false;
      clearFallback();
    });
  };

  void setupPreviewListener();

  return {
    appSettings,
    errorMessage,
    soundError,
    isSoundSaving,
    isPreviewing,
    previewText,
    clearSoundError,
    refresh,
    setDefaultMessage,
    setDefaultSound,
    selectDefaultSound,
    clearDefaultSound,
    previewDefaultSound,
    flushSave,
  };
};

let state: ReturnType<typeof createState> | null = null;

export const useAppSettings = () => {
  if (!state) {
    state = createState();
  }
  return state;
};