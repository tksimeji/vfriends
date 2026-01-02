<script setup lang="ts">
import {computed} from 'vue';
import UserAvatar from '../../components/UserAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcSelect from '../../components/VrcSelect.vue';
import type {VRChat} from '../../vrchat.ts';
import SettingsCard from './SettingsCard.vue';
import {useI18n} from 'vue-i18n';
import {setLocale, type LocaleKey} from '../../i18n';

const props = defineProps<{
  authedUser: VRChat.CurrentUser | null;
  messageTemplate: string;
  previewText: string;
  soundLabel: string;
  errorMessage: string | null;
}>();

const emit = defineEmits<{
  (e: 'update:messageTemplate', value: string): void;
  (e: 'clear-sound'): void;
  (e: 'logout'): void;
  (e: 'preview-sound'): void;
  (e: 'select-sound', file: File | null): void;
}>();

const {t, locale} = useI18n();

const languageOptions = computed(() => [
  {value: 'ja', label: t('settings.languageOptions.ja')},
  {value: 'en', label: t('settings.languageOptions.en')},
]);

const handleLanguageChange = (value: string) => {
  setLocale(value as LocaleKey);
};
</script>

<template>
  <div class="flex flex-col gap-4 p-5">
    <SettingsCard :title="t('settings.accountTitle')">
      <div class="flex flex-col gap-2 items-center">
        <UserAvatar
            v-if="authedUser"
            :user="authedUser"
            :size="128"
            fallback-class="font-semibold text-[12px]"
        />
        <div class="flex flex-col items-center text-vrc-text">
          <p class="font-bold text-2xl">{{ authedUser?.displayName ?? t('common.vrchatUser') }}</p>
          <p v-if="authedUser?.username" class="text-sm text-vrc-text/60">
            {{ authedUser.username }}
          </p>
        </div>
      </div>
      <div class="flex justify-center">
        <VrcButton
            size="sm"
            class="bg-red-500/10! border-red-500/40! text-red-200! hover:bg-red-500/20! hover:border-red-400! hover:text-red-100!"
            @click="emit('logout')"
        >
          {{ t('common.logoutAction') }}
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

    <SettingsCard :title="t('settings.notificationsTitle')">
      <p class="text-sm text-vrc-text/70">
        {{ t('settings.notificationsDescription') }}
      </p>

      <VrcInput
          :label="t('settings.notificationMessageLabel')"
          :value="messageTemplate"
          :placeholder="t('settings.notificationMessagePlaceholder')"
          @input="emit('update:messageTemplate', ($event.target as HTMLInputElement).value)"
      />
      <p class="text-vrc-text/70 text-xs">{{ t('common.preview', {text: previewText}) }}</p>

      <VrcFilePicker
          :label="t('settings.notificationSoundLabel')"
          :value="soundLabel"
          :helper="t('settings.notificationSoundHelper')"
          :clearable="true"
          accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
          @select="(file) => emit('select-sound', file)"
          @clear="emit('clear-sound')"
      />
      <div class="flex flex-wrap gap-2">
        <VrcButton size="sm" @click="emit('preview-sound')">{{ t('common.testSound') }}</VrcButton>
      </div>
      <div class="flex gap-3 items-center">
        <span v-if="errorMessage" class="text-red-300 text-xs">{{ errorMessage }}</span>
      </div>
    </SettingsCard>
  </div>
</template>
