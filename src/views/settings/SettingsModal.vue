<script setup lang="ts">
import {computed, nextTick, ref, watch} from 'vue';
import {VueFinalModal} from 'vue-final-modal';
import {useI18n} from 'vue-i18n';
import type {VRChat} from '../../vrchat.ts';
import AppSettingsPage from './AppSettingsPage.vue';
import FriendSettingsPage from './FriendSettingsPage.vue';
import SettingsSidebar from './SettingsSidebar.vue';

const props = defineProps<{
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
const panelScrollRef = ref<HTMLElement | null>(null);

const activeFriendId = computed(() =>
    settingsTarget.value === 'global' ? null : settingsTarget.value,
);
const activeFriend = computed(() =>
    activeFriendId.value
        ? props.friends.find((friend) => friend.id === activeFriendId.value) ?? null
        : null,
);
const isGlobalView = computed(() => settingsTarget.value === 'global');
const panelKey = computed(() => isGlobalView.value ? 'global' : activeFriendId.value ?? 'global');
const panelScrollClass = computed(() => isPanelAnimating.value ? 'h-full overflow-hidden' : 'h-full overflow-y-auto');
const {t} = useI18n();

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

watch(
    () => panelKey.value,
    async () => {
      await nextTick();
      panelScrollRef.value?.scrollTo({top: 0, behavior: 'auto'});
    },
);

const selectGlobalSettings = () => {
  settingsTarget.value = 'global';
};

const selectFriendSettings = (friendId: string) => {
  settingsTarget.value = friendId;
};

const clearScrollTarget = () => {
  scrollTargetId.value = null;
};

const openGlobal = () => {
  settingsTarget.value = 'global';
  scrollTargetId.value = null;
  isOpen.value = true;
};

const openFriend = (friendId: string) => {
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
    <div
        class="bg-vrc-background border-2 border-vrc-highlight/80 flex pointer-events-auto w-full md:h-[calc(90vh-3rem)] md:max-w-6xl md:rounded-md md:w-[96vw] md:mx-auto">
      <SettingsSidebar
          :friends="props.friends"
          :selected-id="settingsTarget"
          :scroll-target-id="scrollTargetId"
          @select="(id) => {
          clearScrollTarget();
          id === 'global' ? selectGlobalSettings() : selectFriendSettings(id);
        }"
          @scrolled="clearScrollTarget"
      />
      <section class="flex-1 min-h-0 overflow-hidden">
        <div ref="panelScrollRef" :class="panelScrollClass">
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
                  @logout="emit('logout')"
              />
              <FriendSettingsPage
                  v-else-if="activeFriend"
                  :friend="activeFriend"
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
