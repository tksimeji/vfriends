<script setup lang="ts">
import {computed, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import StatusBadge from '../../components/StatusBadge.vue';
import UserAvatar from '../../components/UserAvatar.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcToggle from '../../components/VrcToggle.vue';
import {useDominantColor} from '../../composables/useDominantColor';
import {fetchFriendSettings, setFriendSettings} from '../../invokes';
import {FriendSettings} from '../../types.ts';
import {resolveSoundPath, soundLabel} from '../../utils/notificationSound';
import {VRChat} from '../../vrchat.ts';
import NotificationPreview from './NotificationPreview.vue';
import SettingsCard from './SettingsCard.vue';
import SettingsRow from './SettingsRow.vue';
import type {FriendSettingsContext} from './types';

const props = defineProps<{
  friend: VRChat.LimitedUserFriend;
  context: FriendSettingsContext;
}>();

const emit = defineEmits<{
  (e: 'updated'): void;
}>();

const {t} = useI18n();

const settings = ref<FriendSettings | null>(null);

const loadSettings = async () => {
  const map = await fetchFriendSettings();
  settings.value = map?.[props.friend.id] ?? null;
};

onMounted(() => {
  void loadSettings();
});

watch(
    () => props.friend.id,
    () => {
      void loadSettings();
    },
);

const friendSource = computed(() => props.friend);
const {overlayStyle} = useDominantColor(friendSource);

const notificationsEnabled = computed(() => settings.value?.enabled !== false);
const customizeEnabled = computed(() => settings.value?.useOverride ?? false);
const canCustomize = computed(
    () => notificationsEnabled.value && customizeEnabled.value,
);

const messageDraft = ref('');
const soundDraft = ref('');

watch(
    () => settings.value,
    (next) => {
      messageDraft.value = next?.messageOverride ?? '';
      soundDraft.value = next?.soundOverride ?? '';
    },
    {immediate: true},
);

const displayedSoundLabel = computed(() =>
    soundLabel(soundDraft.value || props.context.globalSound),
);

const patchSettings = async (patch: Partial<FriendSettings>) => {
  await setFriendSettings(props.friend.id, patch);
  await loadSettings();
  emit('updated');
};

const handleToggleNotifications = (value: boolean) => {
  void patchSettings({enabled: value});
};

const handleToggleCustomize = (value: boolean) => {
  void patchSettings({useOverride: value});
};

const handleMessageInput = (event: Event) => {
  messageDraft.value = (event.target as HTMLInputElement).value;
};

const commitMessage = () => {
  // messageDraft.value = messageDraft.value;
  const trimmed = messageDraft.value.trim();
  void patchSettings({messageOverride: trimmed ?? null});
};

const handleSelectSound = async (file: File | null) => {
  const path = await resolveSoundPath(file);
  if (!path) return;
  soundDraft.value = path;
  await patchSettings({soundOverride: path});
};

const handleClearSound = () => {
  soundDraft.value = '';
  void patchSettings({soundOverride: null});
};
</script>

<template>
  <div class="min-h-full p-5" :style="overlayStyle">
    <div class="flex flex-col gap-4 select-none">
      <div class="flex gap-3 items-center">
        <UserAvatar :user="friend" :size="48"/>
        <div>
          <p class="font-bold text-xl text-vrc-friend">{{ friend.displayName }}</p>
          <StatusBadge
              class="gap-2 text-vrc-text text-xs"
              label-class="text-vrc-text text-xs"
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
                notificationsEnabled
                    ? t('settings.friend.deliveryOn')
                    : t('settings.friend.deliveryOff')
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
                @update:model-value="handleToggleCustomize"
            />
          </template>
        </SettingsRow>

        <div class="flex flex-col gap-4" :class="canCustomize ? '' : 'opacity-50'">
          <VrcInput
              :label="t('settings.friend.messageLabel')"
              :value="messageDraft"
              :disabled="!canCustomize"
              :placeholder="props.context.globalMessageTemplate"
              @blur="commitMessage"
              @input="handleMessageInput"
          />

          <VrcFilePicker
              :label="t('settings.friend.soundLabel')"
              :value="displayedSoundLabel"
              :helper="t('settings.friend.soundHelp')"
              :disabled="!canCustomize"
              :clearable="true"
              accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
              @select="handleSelectSound"
              @clear="handleClearSound"
          />
        </div>
      </SettingsCard>
    </div>
  </div>
</template>
