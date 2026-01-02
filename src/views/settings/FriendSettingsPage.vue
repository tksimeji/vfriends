<script setup lang="ts">
import {computed, onMounted, ref, watch} from 'vue';
import StatusBadge from '../../components/StatusBadge.vue';
import UserAvatar from '../../components/UserAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcToggle from '../../components/VrcToggle.vue';
import {useNotificationPreferences} from '../../composables/notifications/useNotificationPreferences';
import {useDominantColor} from '../../composables/useDominantColor';
import {previewNotificationSound} from '../../data/notifications';
import {resolveSoundPath, soundLabel} from '../../utils/notificationSound';
import {VRChat} from '../../vrchat.ts';
import SettingsCard from './SettingsCard.vue';
import SettingsRow from './SettingsRow.vue';
import type {FriendSettingsContext} from './types';
import {useI18n} from 'vue-i18n';

const props = defineProps<{
  friend: VRChat.LimitedUserFriend;
  context: FriendSettingsContext;
}>();

const {
  load,
  isEnabled,
  preferenceFor,
  setEnabled,
  setPreference,
} = useNotificationPreferences();
const {t} = useI18n();

onMounted(() => {
  void load();
});

const friendSource = computed(() => props.friend);
const {overlayStyle} = useDominantColor(friendSource);

const preference = computed(() => preferenceFor(props.friend.id));
const notificationsEnabled = computed(() => isEnabled(props.friend.id));
const customizeEnabled = computed(() => preference.value?.useCustom ?? false);
const canCustomize = computed(
  () => notificationsEnabled.value && customizeEnabled.value,
);

const messageInput = ref('');
const soundInput = ref('');

watch(
  preference,
  (next) => {
    messageInput.value = next?.messageTemplate ?? '';
    soundInput.value = next?.sound ?? '';
  },
  {immediate: true},
);

const messageOverridden = computed(() => {
  if (!customizeEnabled.value) return false;
  return preference.value?.messageTemplate !== undefined &&
    preference.value?.messageTemplate !== null;
});

const soundOverridden = computed(() => {
  if (!customizeEnabled.value) return false;
  return preference.value?.sound !== undefined &&
    preference.value?.sound !== null;
});

const displayedSoundLabel = computed(() =>
  soundLabel(soundInput.value || props.context.globalSound),
);
const currentMessageLabel = computed(() =>
  t('settings.friend.currentLabel', {
    value: t(
      messageOverridden.value
        ? 'settings.friend.currentCustom'
        : 'settings.friend.currentGlobal',
    ),
  }),
);
const currentSoundLabel = computed(() =>
  t('settings.friend.currentLabel', {
    value: t(
      soundOverridden.value
        ? 'settings.friend.currentCustom'
        : 'settings.friend.currentGlobal',
    ),
  }),
);
const updatePreference = (patch: Parameters<typeof setPreference>[1]) =>
  setPreference(props.friend.id, patch);

const handleToggleNotifications = (value: boolean) => {
  void setEnabled(props.friend.id, value);
};

const handleToggleCustomize = (value: boolean) => {
  void updatePreference({useCustom: value});
};

const commitMessage = () => {
  void updatePreference({messageTemplate: messageInput.value});
};

const selectSound = async (file: File | null) => {
  const path = await resolveSoundPath(file);
  if (!path) return;
  soundInput.value = path;
  await updatePreference({sound: path});
};

const clearSound = () => {
  soundInput.value = '';
  void updatePreference({sound: ''});
};

const previewSound = async () => {
  const value = soundInput.value.trim() || props.context.globalSound;
  await previewNotificationSound(value ? value : null);
};
</script>

<template>
  <div class="min-h-full p-5" :style="overlayStyle">
    <div class="flex flex-col gap-4">
      <div class="flex gap-3 items-center">
        <UserAvatar :user="friend" :size="48"/>
        <div>
          <p class="font-bold text-xl text-vrc-friend">{{ friend.displayName }}</p>
          <StatusBadge
              class="gap-2 text-vrc-text/70 text-xs"
              label-class="text-vrc-text/70 text-xs"
              :size="12"
              :friend="friend"
          />
        </div>
      </div>

      <SettingsCard :title="t('settings.friend.deliveryTitle')">
        <SettingsRow>
          <template #description>
            <p class="text-sm text-vrc-text">
              {{
                notificationsEnabled ? t('settings.friend.deliveryOn') : t('settings.friend.deliveryOff')
              }}
            </p>
          </template>
          <template #control>
            <VrcToggle
                :model-value="notificationsEnabled"
                @update:model-value="handleToggleNotifications"
            />
          </template>
        </SettingsRow>
      </SettingsCard>

      <SettingsCard :title="t('settings.friend.customizeTitle')">
        <SettingsRow>
          <template #description>
            <div class="space-y-1">
              <p class="text-sm text-vrc-text/70">
                {{
                  customizeEnabled ? t('settings.friend.customizeOn') : t('settings.friend.customizeOff')
                }}
              </p>
            </div>
          </template>
          <template #control>
            <VrcToggle
                :model-value="customizeEnabled"
                :disabled="!notificationsEnabled"
                @update:model-value="handleToggleCustomize"
            />
          </template>
        </SettingsRow>

        <div :class="canCustomize ? '' : 'opacity-50'">
          <VrcInput
              :label="t('settings.friend.messageLabel')"
              :value="messageInput"
              :disabled="!canCustomize"
              :placeholder="props.context.globalMessageTemplate"
              @blur="commitMessage"
              @input="messageInput = ($event.target as HTMLInputElement).value"
          />
          <p class="mt-1 text-vrc-text/70 text-xs">{{ t('settings.friend.messageHelp') }}</p>
          <p class="text-[10px] text-vrc-text/60">
            {{ currentMessageLabel }}
          </p>

          <VrcFilePicker
              :label="t('settings.friend.soundLabel')"
              :value="displayedSoundLabel"
              :helper="t('settings.friend.soundHelp')"
              :disabled="!canCustomize"
              :clearable="true"
              accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
              @select="selectSound"
              @clear="clearSound"
          />
          <p class="text-[10px] text-vrc-text/60">
            {{ currentSoundLabel }}
          </p>
          <div class="flex flex-wrap gap-2 mt-2">
            <VrcButton
                size="sm"
                :disabled="!canCustomize"
                @click="previewSound"
            >
              {{ t('common.testSound') }}
            </VrcButton>
          </div>
        </div>
      </SettingsCard>
    </div>
  </div>
</template>
