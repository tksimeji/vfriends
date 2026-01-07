import {computed, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {fetchAppSettings, previewNotificationSound, setAppSettings} from '../invokes';
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
  let saveTimer: ReturnType<typeof setTimeout> | null = null;
  let soundErrorTimer: ReturnType<typeof setTimeout> | null = null;

  const previewText = computed(() =>
    appSettings.value.defaultMessage
      .trim()
      .replace('{name}', t('settings.vrchatUserFallback'))
      .replace('{displayName}', t('settings.vrchatUserFallback')) ||
    t('settings.notifications.defaultMessageTemplate'),
  );

  const saveNow = async () => {
    try {
      await setAppSettings({
        defaultMessage: appSettings.value.defaultMessage,
        defaultSound: appSettings.value.defaultSound ?? '',
      });
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

  const previewSound = async (value?: string | null) => {
    try {
      const resolved = value?.trim();
      await previewNotificationSound(resolved ? resolved : null);
    } catch (error) {
      console.error(error);
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

  return {
    appSettings,
    errorMessage,
    soundError,
    isSoundSaving,
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
