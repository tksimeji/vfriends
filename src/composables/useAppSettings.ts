import {computed, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import {fetchAppSettings, previewNotificationSound, setAppSettings} from '../invokes';
import {AppSettings} from '../types.ts';
import {resolveSoundPath} from '../utils.ts';

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
  const isReady = ref(false);
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  const previewText = computed(() =>
    appSettings.value.defaultMessage
      .trim()
      .replace('{name}', t('settings.vrchatUserFallback'))
      .replace('{displayName}', t('settings.vrchatUserFallback')) ||
    t('settings.notifications.defaultMessageTemplate'),
  );

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
      void setAppSettings({
        defaultMessage: appSettings.value.defaultMessage,
        defaultSound: appSettings.value.defaultSound ?? '',
      }).catch((error) => {
        console.error(error);
        errorMessage.value = t('settings.notifications.errors.saveFailed');
      });
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

  const selectDefaultSound = async (file: File | null) => {
    try {
      const path = await resolveSoundPath(file);
      if (!path) return;
      appSettings.value.defaultSound = path;
    } catch (error) {
      console.error(error);
    }
  };

  const clearDefaultSound = () => {
    appSettings.value.defaultSound = null;
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

  return {
    appSettings,
    errorMessage,
    previewText,
    refresh,
    setDefaultMessage,
    setDefaultSound,
    selectDefaultSound,
    clearDefaultSound,
    previewDefaultSound,
  };
};

let state: ReturnType<typeof createState> | null = null;

export const useAppSettings = () => {
  if (!state) {
    state = createState();
  }
  return state;
};
