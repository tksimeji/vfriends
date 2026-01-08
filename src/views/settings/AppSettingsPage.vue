<script setup lang="ts">
import {getVersion} from '@tauri-apps/api/app';
import {openUrl} from '@tauri-apps/plugin-opener';
import {PlayIcon} from 'lucide-vue-next';
import {computed, onMounted, ref} from 'vue';
import {useI18n} from 'vue-i18n';
import DiscordJoinButton from '../../components/DiscordJoinButton.vue';
import ShareOnTwitterButton from '../../components/ShareOnTwitterButton.vue';
import VrcAvatar from '../../components/VrcAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcSelect from '../../components/VrcSelect.vue';
import {useAppSettings} from '../../composables/useAppSettings.ts';
import {useAuthSession} from '../../composables/useAuthSession';
import {type LocaleKey, setLocale} from '../../i18n';
import LogoutConfirmButton from './LogoutConfirmButton.vue';
import SettingsCard from './SettingsCard.vue';

const appVersion = ref('');

const emit = defineEmits<{
  (e: 'logout'): void;
}>();

const {t, locale} = useI18n();
const {currentUser} = useAuthSession();
const {
  appSettings,
  errorMessage,
  soundError,
  isSoundSaving,
  isPreviewing,
  previewText,
  clearSoundError,
  refresh,
  selectDefaultSound,
  clearDefaultSound,
  previewDefaultSound,
  setDefaultMessage,
  flushSave,
} = useAppSettings();

const languageOptions = computed(() => [
  {value: 'ja', label: t('settings.languageOptions.ja')},
  {value: 'en', label: t('settings.languageOptions.en')},
]);
const displayName = computed(() => currentUser.value?.displayName ?? t('settings.vrchatUserFallback'));
const username = computed(() => currentUser.value?.username ?? '');
const showUsername = computed(() => Boolean(currentUser.value?.username));
const versionLabel = computed(() => appVersion.value ? `v${appVersion.value}` : '—');

const loadVersion = async () => {
  try {
    appVersion.value = await getVersion();
  } catch (error) {
    console.warn(error);
  }
};

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

const commitPending = () => {
  void flushSave();
};

onMounted(() => {
  void refresh();
  void loadVersion();
});

defineExpose({
  commitPending,
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
      <LogoutConfirmButton @confirm="emit('logout')"/>
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
      <p class="text-vrc-text text-xs">{{ t('settings.friend.messageHelper', {text: previewText}) }}</p>

      <VrcFilePicker
          :label="t('settings.notifications.soundLabel')"
          :value="appSettings.defaultSound"
          :error="soundError"
          :disabled="isSoundSaving"
          :clearable="true"
          accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
          @select="handleSelectSound"
          @clear="handleClearSound"
          @clear-error="clearSoundError"
      />
      <div class="flex flex-wrap gap-2">
        <VrcButton size="sm" :disabled="isPreviewing" :loading="isPreviewing" @click="handlePreviewSound">
          {{ t('settings.notifications.testSound') }}
          <PlayIcon :size="14"/>
        </VrcButton>
      </div>
      <div class="flex gap-3 items-center">
        <span v-if="isSoundSaving" class="text-vrc-text/70 text-xs">
          {{ t('settings.notifications.savingSound') }}
        </span>
      </div>
      <div
          v-if="errorMessage"
          class="bg-red-500/10 border border-red-400/40 px-3 py-2 rounded-md text-red-200 text-xs"
      >
        {{ errorMessage }}
      </div>
    </SettingsCard>

    <SettingsCard :title="t('settings.about.title')">
      <div class="space-y-8">
        <div>
          <p class="select-text text-vrc-text text-sm">
            {{ t('settings.about.versionLabel') }} {{ versionLabel }}
          </p>
          <a
              class="duration-150 flex gap-2 items-center transition-colors underline hover:text-vrc-highlight"
              @click="() => openUrl('https://tksimeji.booth.pm/items/7844126')"
          >
            <img
                class="select-none size-6"
                src="../../assets/Booth.png"
                alt="Booth"
                :draggable="false"
            />
            <span>{{ t('settings.about.boothLatest') }}</span>
          </a>
        </div>

        <div class="space-y-2">
          <div class="flex gap-3 items-center">
            <img
                class="rounded-full select-none size-10"
                src="../../assets/tksimeji.png"
                alt="tksimeji's Icon"
                :draggable="false"
            />
            <div class="flex flex-col">
              <p class="font-semibold text-2xl text-vrc-text">
                {{ t('settings.about.developerName') }}
              </p>
            </div>
          </div>
          <p class="text-sm text-[#cccfff]">
            {{ t('settings.about.thanksMessage') }}
          </p>
          <a
              class="flex gap-2 items-center hover:underline"
              @click="() => openUrl('https://x.com/tksimeji')"
          >
            <img
                class="size-4"
                src="../../assets/TwitterBlue.png"
                alt="Twitter"
                :draggable="false"
            />
            <span>Twitter</span>
          </a>
          <a
              class="flex gap-2 items-center hover:underline"
              @click="() => openUrl('https://github.com/tksimeji')"
          >
            <img
                class="size-4"
                src="../../assets/GitHub.png"
                alt="GitHub"
                :draggable="false"
            />
            <span>GitHub</span>
          </a>
        </div>

        <div class="space-y-4">
          <p class="text-sm text-vrc-text">
            {{ t('oobe.discordPrompt') }}
          </p>
          <div class="flex gap-4">
            <DiscordJoinButton/>
            <ShareOnTwitterButton/>
          </div>
        </div>
      </div>
    </SettingsCard>
  </div>
</template>