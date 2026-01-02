<script setup lang="ts">
import {computed, onBeforeUnmount, ref, watch} from 'vue';
import {VueFinalModal} from 'vue-final-modal';
import {useNotificationPreferences} from '../../composables/notifications/useNotificationPreferences';
import {useNotificationSettings} from '../../composables/notifications/useNotificationSettings';
import {previewNotificationSound} from '../../data/notifications';
import {resolveSoundPath, soundLabel} from '../../utils/notificationSound';
import {VRChat} from '../../vrchat.ts';
import AppSettingsPage from './AppSettingsPage.vue';
import FriendSettingsPage from './FriendSettingsPage.vue';
import SettingsSidebar from './SettingsSidebar.vue';
import {useI18n} from 'vue-i18n';

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

const {load: loadPreferences, isEnabled: isNotificationEnabled} = useNotificationPreferences();

const {
  settings,
  load: loadSettings,
  save,
  isLoaded,
  previewText,
  errorMessage,
} = useNotificationSettings();

const activeFriendId = computed(() =>
    settingsTarget.value === 'global' ? null : settingsTarget.value,
);
const activeFriend = computed(() =>
    activeFriendId.value
        ? props.friends.find((friend) => friend.id === activeFriendId.value) ?? null
        : null,
);
const isGlobalView = computed(() => settingsTarget.value === 'global');
const panelKey = computed(() => (isGlobalView.value ? 'global' : activeFriendId.value ?? 'global'));
const panelScrollClass = computed(() =>
  isPanelAnimating.value ? 'h-full overflow-hidden' : 'h-full overflow-y-auto',
);
const {t} = useI18n();

// Friend settings updates are handled inside FriendSettingsPage.

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
    if (path) settings.value.sound = path;
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
  await previewSound(settings.value.sound);
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

let autoSaveTimer: ReturnType<typeof setTimeout> | null = null;

const scheduleAutoSave = () => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
  autoSaveTimer = setTimeout(() => {
    void save();
  }, 500);
};

watch(
    () => [settings.value.messageTemplate, settings.value.sound],
    (next, prev) => {
      if (!isLoaded.value) return;
      if (next[0] === prev[0] && next[1] === prev[1]) return;
      scheduleAutoSave();
    },
);

onBeforeUnmount(() => {
  if (autoSaveTimer) {
    clearTimeout(autoSaveTimer);
  }
});

const openGlobal = async () => {
  await Promise.all([loadPreferences(), loadSettings()]);
  settingsTarget.value = 'global';
  scrollTargetId.value = null;
  isOpen.value = true;
};

const openFriend = async (friendId: string) => {
  await Promise.all([loadPreferences(), loadSettings()]);
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
          :is-notification-enabled="isNotificationEnabled"
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
                  :message-template="settings.messageTemplate"
                  :preview-text="previewText"
                  :sound-label="soundLabel(settings.sound)"
                  @clear-sound="settings.sound = ''"
                  @logout="emit('logout')"
                  @preview-sound="previewGlobalSound"
                  @select-sound="handleGlobalSoundSelected"
                  @update:message-template="(value) => (settings.messageTemplate = value)"
              />
              <FriendSettingsPage
                  v-else-if="activeFriend"
                  :friend="activeFriend"
                  :context="{
                    globalMessageTemplate: settings.messageTemplate,
                    globalSound: settings.sound,
                  }"
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
