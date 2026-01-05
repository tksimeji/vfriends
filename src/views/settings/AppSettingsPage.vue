<script setup lang="ts">
import {computed, onMounted} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcAvatar from '../../components/VrcAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcSelect from '../../components/VrcSelect.vue';
import {useAppSettings} from '../../composables/useAppSettings.ts';
import {useAuthSession} from '../../composables/useAuthSession';
import {type LocaleKey, setLocale} from '../../i18n';
import SettingsCard from './SettingsCard.vue';

const emit = defineEmits<{
  (e: 'logout'): void;
}>();

const {t, locale} = useI18n();
const {currentUser} = useAuthSession();
const {
  appSettings,
  errorMessage,
  previewText,
  refresh,
  selectDefaultSound,
  clearDefaultSound,
  previewDefaultSound,
  setDefaultMessage,
} = useAppSettings();

const languageOptions = computed(() => [
  {value: 'ja', label: t('settings.languageOptions.ja')},
  {value: 'en', label: t('settings.languageOptions.en')},
]);
const displayName = computed(() => currentUser.value?.displayName ?? t('settings.vrchatUserFallback'));
const username = computed(() => currentUser.value?.username ?? '');
const showUsername = computed(() => Boolean(currentUser.value?.username));

const handleLanguageChange = (value: string) => {
  setLocale(value as LocaleKey);
};

const handleMessageInput = (event: Event) => {
  setDefaultMessage((event.target as HTMLInputElement).value);
};

const handleSelectSound = (file: File | null) => {
  void selectDefaultSound(file);
};

const handleClearSound = () => {
  clearDefaultSound();
};

const handlePreviewSound = () => {
  void previewDefaultSound();
};

onMounted(() => {
  void refresh();
});
</script>

<template>
  <div class="flex flex-col gap-4 p-5">
    <SettingsCard :title="t('settings.accountTitle')">
      <div class="flex flex-col gap-2 items-center">
        <VrcAvatar
            v-if="currentUser"
            :user="currentUser"
            :size="128"
            fallback-class="font-semibold text-[12px]"
        />
        <div class="flex flex-col items-center text-vrc-text">
          <p class="font-bold text-2xl">{{ displayName }}</p>
          <p v-if="showUsername" class="text-sm text-vrc-text">
            {{ username }}
          </p>
        </div>
      </div>
      <div class="flex justify-center">
        <VrcButton
            size="sm"
            class="bg-red-500/10! border-red-500/40! text-red-200! hover:bg-red-500/20! hover:border-red-400! hover:text-red-100!"
            @click="emit('logout')"
        >
          {{ t('settings.actions.logout') }}
        </VrcButton>
      </div>
    </SettingsCard>

    <SettingsCard :title="t('settings.languageTitle')">
      <VrcSelect
          :helper="t('settings.languageHelper')"
          :label="t('settings.languageLabel')"
          :model-value="locale"
          :options="languageOptions"
          @update:model-value="handleLanguageChange"
      />
    </SettingsCard>

    <SettingsCard :title="t('settings.notifications.title')">
      <p class="text-sm text-vrc-text">
        {{ t('settings.notifications.description') }}
      </p>

      <VrcInput
          :label="t('settings.notifications.messageLabel')"
          :value="appSettings.defaultMessage"
          :placeholder="t('settings.notifications.messagePlaceholder')"
          @input="handleMessageInput"
      />
      <p class="text-vrc-text text-xs">{{ t('settings.notifications.preview', {text: previewText}) }}</p>

      <VrcFilePicker
          :label="t('settings.notifications.soundLabel')"
          :value="appSettings.defaultSound"
          :clearable="true"
          accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
          @select="handleSelectSound"
          @clear="handleClearSound"
      />
      <div class="flex flex-wrap gap-2">
        <VrcButton size="sm" @click="handlePreviewSound">{{ t('settings.notifications.testSound') }}</VrcButton>
      </div>
      <div class="flex gap-3 items-center">
        <span v-if="errorMessage" class="text-red-300 text-xs">{{ errorMessage }}</span>
      </div>
    </SettingsCard>
  </div>
</template>
