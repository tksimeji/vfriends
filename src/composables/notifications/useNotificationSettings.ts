import {computed, ref} from 'vue';
import {fetchNotificationSettings, setNotificationSettings} from '../../data/notifications';
import type {NotificationConfig} from '../../domain/notifications';
import {t} from '../../i18n';

const defaultTemplate = () => t('notifier.defaultMessageTemplate');

const settings = ref<NotificationConfig>({
  messageTemplate: defaultTemplate(),
  sound: '',
});
const isLoaded = ref(false);
const isSaving = ref(false);
const errorMessage = ref('');
let loadPromise: Promise<void> | null = null;

const load = async () => {
  if (isLoaded.value) return;
  if (loadPromise) return loadPromise;
  loadPromise = (async () => {
    try {
      const result = await fetchNotificationSettings();
      settings.value = {
        messageTemplate: result?.messageTemplate ?? defaultTemplate(),
        sound: result?.sound ?? '',
      };
      errorMessage.value = '';
    } catch (error) {
      console.error(error);
      errorMessage.value = t('notifier.errors.loadFailed');
    } finally {
      isLoaded.value = true;
      loadPromise = null;
    }
  })();
  return loadPromise;
};

const save = async () => {
  if (isSaving.value) return;
  isSaving.value = true;
  try {
    await setNotificationSettings({
      messageTemplate: settings.value.messageTemplate,
      sound: settings.value.sound || '',
    });
    errorMessage.value = '';
  } catch (error) {
    console.error(error);
    errorMessage.value = t('notifier.errors.saveFailed');
  } finally {
    isSaving.value = false;
  }
};

const previewText = computed(() =>
  settings.value.messageTemplate
    .trim()
    .replace('{name}', t('common.vrchatUser'))
    .replace('{displayName}', t('common.vrchatUser')) || defaultTemplate(),
);

export const useNotificationSettings = () => {
  if (!isLoaded.value && !loadPromise) {
    void load();
  }

  return {
    settings,
    isLoaded,
    isSaving,
    errorMessage,
    load,
    save,
    previewText,
  };
};
