<script setup lang="ts">
import {computed, ref} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import type {AuthUser} from './composables/useAuthFlow.ts';
import type {VRChat} from './vrchat.ts';
import AuthModal from './views/auth/AuthModal.vue';
import FriendsView from './views/friends/FriendsView.vue';
import TitleBar from './components/TitleBar.vue';
import './style.css';
import 'vue-final-modal/style.css';

const authedUser = ref<AuthUser | null>(null);
const isAuthenticated = computed(() => Boolean(authedUser.value));
const searchQuery = ref('');
const searchSuggestions = ref<VRChat.LimitedUserFriend[]>([]);
const isSettingsOpen = ref(false);
const hoverColor = ref<[number, number, number] | null>(null);

type FriendsViewHandle = {
  openSettings: () => void;
  openSettingsForFriend: (friendId: string) => void;
  closeSettings: () => void;
};

const friendsViewRef = ref<FriendsViewHandle | null>(null);

const handleLoginSuccess = (user: AuthUser | null) => {
  authedUser.value = user;
};

const handleLogout = () => {
  authedUser.value = null;
  searchSuggestions.value = [];
  hoverColor.value = null;
};

const handleSuggestionsUpdated = (
  suggestions: VRChat.LimitedUserFriend[],
) => {
  searchSuggestions.value = suggestions;
};

const handleOpenSettings = () => {
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
          :suggestions="searchSuggestions"
          :hide-search="isSettingsOpen"
          @open-settings="handleOpenSettings"
          @open-friend-settings="handleOpenFriendSettings"
          @logout="handleLogoutFromTitle"
      />
    </Teleport>

    <div class="flex flex-1 flex-col items-center min-h-0 overflow-hidden relative z-10">
      <FriendsView
          ref="friendsViewRef"
          :search-query="searchQuery"
          @hover-color="handleHoverColor"
          @suggestions-updated="handleSuggestionsUpdated"
          @settings-opened="isSettingsOpen = true"
          @settings-closed="isSettingsOpen = false"
      />

      <div
          v-if="!isAuthenticated"
          class="absolute backdrop-blur-md bg-black/40 flex inset-0 items-center justify-center px-6 py-10"
      >
        <AuthModal @login-success="handleLoginSuccess"/>
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

