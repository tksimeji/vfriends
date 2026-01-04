<script setup lang="ts">
import {computed, ref, watch} from 'vue';
import {VueFinalModal} from 'vue-final-modal';
import {useI18n} from 'vue-i18n';
import {fetchAppSettings, fetchFriendSettings, previewNotificationSound, setAppSettings} from '../../invokes';
import {AppSettings, FriendSettings} from '../../types.ts';
import {resolveSoundPath, soundLabel} from '../../utils/notificationSound';
import {VRChat} from '../../vrchat.ts';
import AppSettingsPage from './AppSettingsPage.vue';
import FriendSettingsPage from './FriendSettingsPage.vue';
import SettingsSidebar from './SettingsSidebar.vue';

const props = defineProps<{
  currentUser: VRChat.CurrentUser | null;
  friends: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'open'): void;
  (e: 'close'): void;
  (e: 'logout'): void;
}>();

const isOpen = ref(false);
const settingsTarget = ref<'global' | string>('global');
const scrollTargetId = ref<string | null>(null);
const isPanelAnimating = ref(false);

const appSettings = ref<AppSettings>({
  defaultMessage: '',
  defaultSound: null,
  friendSettings: {},
});
const friendSettings = ref<Record<string, FriendSettings>>({});
const errorMessage = ref('');
let saveTimer: ReturnType<typeof setTimeout> | null = null;

const activeFriendId = computed(() =>
  settingsTarget.value === 'global' ? null : settingsTarget.value,
);
const activeFriend = computed(() =>
    activeFriendId.value
        ? props.friends.find((friend) => friend.id === activeFriendId.value) ?? null
        : null,
);
const isGlobalView = computed(() => settingsTarget.value === 'global');
const panelKey = computed(() =>
  isGlobalView.value ? 'global' : activeFriendId.value ?? 'global',
);
const panelScrollClass = computed(() =>
  isPanelAnimating.value ? 'h-full overflow-hidden' : 'h-full overflow-y-auto',
);
const {t} = useI18n();

// Friend config updates are handled inside FriendSettingsPage.

const previewText = computed(() =>
  appSettings.value.defaultMessage
    .trim()
    .replace('{name}', t('common.vrchatUser'))
    .replace('{displayName}', t('common.vrchatUser')) || t('notifications.defaultMessageTemplate'),
);

const refreshAppSettings = async () => {
  try {
    const result = await fetchAppSettings();
    appSettings.value = {
      defaultMessage: result?.defaultMessage ?? t('notifications.defaultMessageTemplate'),
      defaultSound: result?.defaultSound ?? null,
      friendSettings: result?.friendSettings ?? {},
    };
    errorMessage.value = '';
  } catch (error) {
    console.error(error);
    errorMessage.value = t('notifications.errors.loadFailed');
  }
};

const refreshFriendSettings = async () => {
  try {
    friendSettings.value = await fetchFriendSettings();
  } catch (error) {
    console.error(error);
  }
};

watch(
    () => props.friends,
    (friends) => {
      if (settingsTarget.value === 'global') return;
      const exists = friends.some((friend) => friend.id === settingsTarget.value);
      if (!exists) {
        settingsTarget.value = 'global';
      }
    },
);

const handleGlobalSoundSelected = async (file: File | null) => {
  try {
    const path = await resolveSoundPath(file);
    if (path) appSettings.value.defaultSound = path;
  } catch (error) {
    console.error(error);
  }
};

const previewSound = async (value?: string | null) => {
  try {
    const resolved = value?.trim();
    await previewNotificationSound(resolved ? resolved : null);
  } catch (error) {
    console.error(error);
  }
};

const previewGlobalSound = async () => {
  await previewSound(appSettings.value.defaultSound);
};

const selectGlobalSettings = () => {
  settingsTarget.value = 'global';
};

const selectFriendSettings = (friendId: string) => {
  settingsTarget.value = friendId;
};

const clearScrollTarget = () => {
  scrollTargetId.value = null;
};

const openGlobal = async () => {
  await refreshAppSettings();
  await refreshFriendSettings();
  settingsTarget.value = 'global';
  scrollTargetId.value = null;
  isOpen.value = true;
};

const openFriend = async (friendId: string) => {
  await refreshAppSettings();
  await refreshFriendSettings();
  settingsTarget.value = friendId;
  scrollTargetId.value = friendId;
  isOpen.value = true;
};

const close = () => {
  isOpen.value = false;
};

watch(isOpen, (next, prev) => {
  if (next === prev) return;
  if (next) {
    emit('open');
  } else {
    emit('close');
  }
});

watch(
  () => [appSettings.value.defaultMessage, appSettings.value.defaultSound],
  (next, prev) => {
    if (next[0] === prev[0] && next[1] === prev[1]) return;
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => {
      void setAppSettings({
        defaultMessage: appSettings.value.defaultMessage,
        defaultSound: appSettings.value.defaultSound ?? '',
      }).catch((error) => {
        console.error(error);
        errorMessage.value = t('notifications.errors.saveFailed');
      });
    }, 500);
  },
);

defineExpose({
  openGlobal,
  openFriend,
  close,
});

const handlePanelEnter = () => {
  isPanelAnimating.value = true;
};

const handlePanelLeave = () => {
  isPanelAnimating.value = true;
};

const handlePanelAfterEnter = () => {
  isPanelAnimating.value = false;
};

const handlePanelAfterLeave = () => {
  isPanelAnimating.value = false;
};
</script>

<template>
  <VueFinalModal
      v-model="isOpen"
      :z-index-fn="() => 40"
      overlay-transition="vfm-fade"
      content-transition="vfm-slide-up"
      overlay-class="backdrop-blur-sm"
      content-class="flex h-full items-center justify-center p-0 pointer-events-none w-full lg:p-6"
      teleport-to="body"
  >
    <div class="bg-vrc-background border-2 border-vrc-highlight/80 flex pointer-events-auto w-full md:h-[calc(90vh-3rem)] md:max-w-6xl md:rounded-md md:w-[96vw] md:mx-auto">
    <SettingsSidebar
        :current-user="props.currentUser"
        :friends="props.friends"
        :selected-id="settingsTarget"
        :scroll-target-id="scrollTargetId"
        :is-friend-enabled="(friendId) => friendSettings[friendId]?.enabled !== false"
        @select="(id) => {
          clearScrollTarget();
          id === 'global' ? selectGlobalSettings() : selectFriendSettings(id);
        }"
        @scrolled="clearScrollTarget"
    />
      <section class="flex-1 min-h-0 overflow-hidden">
        <div :class="panelScrollClass">
          <Transition
              class="h-full"
              name="settings-panel"
              mode="out-in"
              @before-enter="handlePanelEnter"
              @before-leave="handlePanelLeave"
              @after-enter="handlePanelAfterEnter"
              @after-leave="handlePanelAfterLeave"
          >
            <div :key="panelKey" class="min-h-full">
              <AppSettingsPage
                  v-if="isGlobalView"
                  :authed-user="props.currentUser"
                  :error-message="errorMessage"
                  :default-message="appSettings.defaultMessage"
                  :preview-text="previewText"
                  :sound-label="soundLabel(appSettings.defaultSound)"
                  @clear-sound="appSettings.defaultSound = ''"
                  @logout="emit('logout')"
                  @preview-sound="previewGlobalSound"
                  @select-sound="handleGlobalSoundSelected"
                  @update:default-message="(value) => (appSettings.defaultMessage = value)"
              />
              <FriendSettingsPage
                  v-else-if="activeFriend"
                  :friend="activeFriend"
                  :context="{
                    globalMessageTemplate: appSettings.defaultMessage,
                    globalSound: appSettings.defaultSound ?? '',
                  }"
                  @updated="refreshFriendSettings"
              />
              <div v-else class="p-5 text-sm text-vrc-text/70">
                {{ t('settings.noSettings') }}
              </div>
            </div>
          </Transition>
        </div>
      </section>
    </div>
  </VueFinalModal>
</template>

<style scoped>
.settings-panel-enter-active,
.settings-panel-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.settings-panel-enter-from,
.settings-panel-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
