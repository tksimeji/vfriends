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
const selectedId = ref<'global' | string>('global');
const selectedFriend = ref<VRChat.LimitedUserFriend | null>(null);
const scrollTargetId = ref<string | null>(null);
const isPanelAnimating = ref(false);
const panelScrollRef = ref<HTMLElement | null>(null);
const sidebarRef = ref<{ focusSearch: () => void } | null>(null);
const activeFriend = computed(() => selectedFriend.value);
const isGlobalView = computed(() => selectedId.value === 'global');
const panelKey = computed(() => (isGlobalView.value ? 'global' : selectedId.value));
const panelScrollClass = computed(() => isPanelAnimating.value ? 'h-full overflow-hidden' : 'h-full overflow-y-auto');
const {t} = useI18n();

watch(
    () => panelKey.value,
    async () => {
      await nextTick();
      panelScrollRef.value?.scrollTo({top: 0, behavior: 'auto'});
    },
);

const clearScrollTarget = () => {
  scrollTargetId.value = null;
};

const openGlobal = () => {
  selectedId.value = 'global';
  selectedFriend.value = null;
  scrollTargetId.value = null;
  isOpen.value = true;
};

const openFriend = (friend: VRChat.LimitedUserFriend) => {
  selectedId.value = friend.id;
  selectedFriend.value = friend;
  scrollTargetId.value = friend.id;
  isOpen.value = true;
};

const close = () => {
  isOpen.value = false;
};

const handleSidebarSelect = (payload: {
  id: string;
  friend: VRChat.LimitedUserFriend | null;
}) => {
  clearScrollTarget();
  selectedId.value = payload.id;
  selectedFriend.value = payload.friend;
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
  focusSidebarSearch: () => sidebarRef.value?.focusSearch(),
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
        class="bg-vrc-background border border-vrc-highlight/80 flex pointer-events-auto w-full md:h-[calc(90vh-3rem)] md:max-w-6xl md:rounded-md md:w-[96vw] md:mx-auto">
      <SettingsSidebar
          ref="sidebarRef"
          :friends="props.friends"
          :selected-id="selectedId"
          :scroll-target-id="scrollTargetId"
          @select="handleSidebarSelect"
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