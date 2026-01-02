<script setup lang="ts">
import {invoke} from '@tauri-apps/api/core';
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, onBeforeUnmount, onMounted, ref} from 'vue';
import TitleBar from './components/TitleBar.vue';
import './style.css';
import AuthModal from './views/auth/AuthModal.vue';
import FriendsView from './views/friends/FriendsView.vue';
import type {VRChat} from './vrchat.ts';
import 'vue-final-modal/style.css';
import {useAuthSession} from './composables/useAuthSession';

const {currentUser: authedUser, isAuthenticated, setCurrentUser, clearCurrentUser} =
  useAuthSession();
const searchQuery = ref('');
const isSettingsOpen = ref(false);
const hoverColor = ref<[number, number, number] | null>(null);
const authListener = ref<UnlistenFn | null>(null);
const friendSuggestions = ref<VRChat.LimitedUserFriend[]>([]);

type AuthEvent =
  | { type: 'started'; action: 'credentials' | 'twoFactor' }
  | { type: 'twoFactorRequired'; methods?: string[]; message?: string }
  | { type: 'success'; user?: VRChat.CurrentUser }
  | { type: 'failure'; message: string; code?: string }
  | { type: 'loggedOut' };

type FriendsViewHandle = {
  openSettings: () => void;
  openSettingsForFriend: (friendId: string) => void;
  closeSettings: () => void;
};

const friendsViewRef = ref<FriendsViewHandle | null>(null);

const handleLogout = () => {
  clearCurrentUser();
  hoverColor.value = null;
};

const handleOpenSettings = () => {
  if (isSettingsOpen.value) {
    friendsViewRef.value?.closeSettings();
    return;
  }
  friendsViewRef.value?.openSettings();
};

const handleOpenFriendSettings = (friendId: string) => {
  friendsViewRef.value?.openSettingsForFriend(friendId);
};

const handleHoverColor = (rgb: [number, number, number] | null) => {
  hoverColor.value = rgb;
};

const hoverOverlayKey = computed(() => (hoverColor.value ? hoverColor.value.join('-') : 'none'));
const hoverOverlayStyle = computed(() => {
  if (!hoverColor.value) return {};
  const [r, g, b] = hoverColor.value;
  const accent = `rgba(${r}, ${g}, ${b}, 0.4)`;
  const base = `rgba(31, 35, 42, 0.92)`;
  return {
    backgroundImage: `linear-gradient(135deg, ${accent}, ${base})`,
  };
});

const handleLogoutFromTitle = async () => {
  try {
    await invoke('logout');
  } catch (error) {
    console.error(error);
  } finally {
    friendsViewRef.value?.closeSettings();
    searchQuery.value = '';
    handleLogout();
  }
};

const handleAuthEvent = (event: AuthEvent) => {
  if (event.type === 'success') {
    if (event.user) setCurrentUser(event.user);
    return;
  }
  if (event.type === 'loggedOut') {
    handleLogout();
  }
};

const handleSuggestionsUpdated = (suggestions: VRChat.LimitedUserFriend[]) => {
  friendSuggestions.value = suggestions;
};

onMounted(async () => {
  authListener.value = await listen<AuthEvent>('vrc:auth', (event) => {
    handleAuthEvent(event.payload);
  });

  try {
    const restored = await invoke<VRChat.CurrentUser | null>('restore_session');
    if (restored) {
      setCurrentUser(restored);
    }
  } catch (error) {
    console.error(error);
  }
});

onBeforeUnmount(() => {
  authListener.value?.();
  authListener.value = null;
});
</script>

<template>
  <main
      id="a"
      class="bg-vrc-background flex flex-col h-full overflow-hidden pt-12 relative w-full"
      :class="isAuthenticated ? '' : 'auth-background'"
  >
    <Transition name="hover-overlay">
      <div
          v-if="hoverColor"
          :key="hoverOverlayKey"
          class="absolute inset-0 pointer-events-none z-0"
          :style="hoverOverlayStyle"
      ></div>
    </Transition>
    <Teleport to="#titlebar">
      <TitleBar
          v-model:query="searchQuery"
          :authed-user="authedUser"
          :hide-search="isSettingsOpen"
          :suggestions="friendSuggestions"
          @open-settings="handleOpenSettings"
          @open-friend-settings="handleOpenFriendSettings"
      />
    </Teleport>

    <div class="flex flex-1 flex-col items-center min-h-0 overflow-hidden relative z-10">
      <FriendsView
          ref="friendsViewRef"
          :authed-user="authedUser"
          :search-query="searchQuery"
          @hover-color="handleHoverColor"
          @suggestions-updated="handleSuggestionsUpdated"
          @settings-opened="isSettingsOpen = true"
          @settings-closed="isSettingsOpen = false"
          @logout="handleLogoutFromTitle"
      />

      <div
          v-if="!isAuthenticated"
          class="absolute backdrop-blur-md bg-black/40 flex inset-0 items-center justify-center px-6 py-10"
      >
        <AuthModal />
      </div>
    </div>
  </main>
</template>

<style scoped>
.auth-background {
  background-image: url('./assets/LoginBackground.png');
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;
}

.hover-overlay-enter-active,
.hover-overlay-leave-active {
  transition: opacity 0.5s ease;
}

.hover-overlay-enter-from,
.hover-overlay-leave-to {
  opacity: 0;
}

.hover-overlay-enter-to,
.hover-overlay-leave-from {
  opacity: 1;
}
</style>
