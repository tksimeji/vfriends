<script setup lang="ts">
import {computed, onMounted} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcStatus from '../../components/VrcStatus.vue';
import VrcAvatar from '../../components/VrcAvatar.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcToggle from '../../components/VrcToggle.vue';
import {useDominantColor} from '../../composables/useDominantColor';
import {useAppSettings} from '../../composables/useAppSettings';
import {useFriendSettings} from '../../composables/useFriendSettings';
import {VRChat} from '../../vrchat.ts';
import NotificationPreview from './NotificationPreview.vue';
import SettingsCard from './SettingsCard.vue';
import SettingsRow from './SettingsRow.vue';

const props = defineProps<{
  friend: VRChat.LimitedUserFriend;
}>();

const {t} = useI18n();
const friendSource = computed(() => props.friend);
const {overlayStyle} = useDominantColor(friendSource);
const {appSettings, refresh: refreshAppSettings} = useAppSettings();

const {
  settings,
  messageDraft,
  notificationsEnabled,
  customizeEnabled,
  canCustomize,
  soundDraft,
  handleMessageInput,
  commitMessage,
  selectSound,
  clearSound,
  toggleNotifications,
  toggleCustomize,
} = useFriendSettings({
  friendId: computed(() => props.friend.id),
});

onMounted(() => {
  void refreshAppSettings();
});
</script>

<template>
  <div class="min-h-full p-5" :style="overlayStyle">
    <div class="flex flex-col gap-4 select-none">
      <div class="flex gap-3 items-center">
        <VrcAvatar :user="friend" :size="48"/>
        <div>
          <p class="font-bold text-xl text-vrc-friend">{{ friend.displayName }}</p>
          <VrcStatus
              class="gap-2 text-vrc-text text-xs"
              label-class="text-vrc-text text-xs"
              :size="12"
              :user="friend"
          />
        </div>
      </div>

      <SettingsCard :title="t('settings.friend.deliveryTitle')">
        <SettingsRow>
          <template #description>
            <p class="text-sm text-vrc-text">
              {{
                notificationsEnabled
                    ? t('settings.friend.deliveryOn')
                    : t('settings.friend.deliveryOff')
              }}
            </p>
          </template>
          <template #control>
            <VrcToggle
                :model-value="notificationsEnabled"
                @update:model-value="toggleNotifications"
            />
          </template>
        </SettingsRow>
      </SettingsCard>

      <SettingsCard v-if="notificationsEnabled" :title="t('settings.friend.customizeTitle')">
        <div class="flex justify-center">
          <NotificationPreview :user="props.friend" :settings="settings"/>
        </div>

        <SettingsRow>
          <template #description>
            <div class="space-y-1">
              <p class="text-sm text-vrc-text">
                {{
                  customizeEnabled
                      ? t('settings.friend.customizeOn')
                      : t('settings.friend.customizeOff')
                }}
              </p>
            </div>
          </template>
          <template #control>
            <VrcToggle
                :model-value="customizeEnabled"
                :disabled="!notificationsEnabled"
                @update:model-value="toggleCustomize"
            />
          </template>
        </SettingsRow>

        <div class="flex flex-col gap-4" :class="canCustomize ? '' : 'opacity-50'">
          <VrcInput
              :label="t('settings.friend.messageLabel')"
              :value="messageDraft"
              :disabled="!canCustomize"
              :placeholder="appSettings.defaultMessage"
              @blur="commitMessage"
              @input="handleMessageInput"
          />
          <p class="mt-1 text-vrc-text text-xs">
            {{ t('settings.friend.messageHelper') }}
          </p>

          <VrcFilePicker
              accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
              :label="t('settings.friend.soundLabel')"
              :value="soundDraft"
              :disabled="!canCustomize"
              :clearable="true"
              @select="selectSound"
              @clear="clearSound"
          />
        </div>
      </SettingsCard>
    </div>
  </div>
</template>