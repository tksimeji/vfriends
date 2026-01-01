import {computed, ref} from 'vue';
import {fetchNotificationSettings, setNotificationSettings} from '../../data/notifications';
import type {NotificationSettings} from '../../domain/notifications';

const DEFAULT_TEMPLATE = '{name} is online';

const settings = ref<NotificationSettings>({
  messageTemplate: DEFAULT_TEMPLATE,
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
        messageTemplate: result?.messageTemplate ?? DEFAULT_TEMPLATE,
        sound: result?.sound ?? '',
      };
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
    errorMessage.value = '通知設定の保存に失敗しました。';
  } finally {
    isSaving.value = false;
  }
};

const previewText = computed(() =>
  settings.value.messageTemplate
    .trim()
    .replace('{name}', 'VRChat User')
    .replace('{displayName}', 'VRChat User') || DEFAULT_TEMPLATE,
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
