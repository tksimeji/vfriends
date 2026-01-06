<script setup lang="ts">
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import TitleBar from './components/title/TitleBar.vue';
import './style.css';
import {type AuthEvent, useAuthSession} from './composables/useAuthSession';
import {useDominantColor} from './composables/useDominantColor';
import {logout, restoreSession} from './invokes';
import LoginWelcomeOverlay from './views/auth/LoginWelcomeOverlay.vue';
import FriendsView from './views/friends/FriendsView.vue';
import 'vue-final-modal/style.css';
import Oobe from './views/oobe/Oobe.vue';

type FriendsViewHandle = {
  openSettings: () => void;
  openSettingsForFriend: (friendId: string) => void;
  closeSettings: () => void;
  focusSettingsSearch: () => void;
};

const {
  currentUser,
  isAuthenticated,
  setCurrentUser,
  clearCurrentUser,
  applyAuthEvent,
  isLoginCelebrating
} = useAuthSession();
const {rgb: currentUserRgb} = useDominantColor(currentUser);
const {t} = useI18n();

const searchQuery = ref('');
const isSettingsOpen = ref(false);
const hoverColor = ref<[number, number, number] | null>(null);
const authListener = ref<UnlistenFn | null>(null);
const isAuthChecking = ref(true);
const titleBarRef = ref<{ focusSearch: () => void } | null>(null);
const authOverlayMode = ref<'oobe' | 'auth' | null>(null);
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

const baseOverlayRgb = computed(() => (isAuthenticated.value ? currentUserRgb.value : null));
const overlayRgb = computed(() => hoverColor.value ?? baseOverlayRgb.value);
const hoverOverlayKey = computed(() => (overlayRgb.value ? overlayRgb.value.join('-') : 'none'));
const hoverOverlayStyle = computed(() => {
  if (!overlayRgb.value) return {};
  const [r, g, b] = overlayRgb.value;
  const accent = `rgba(${r}, ${g}, ${b}, 0.4)`;
  const base = `rgba(31, 35, 42, 0.92)`;
  return {
    backgroundImage: `linear-gradient(135deg, ${accent}, ${base})`,
  };
});
const uiState = computed(() => {
  const authed = isAuthenticated.value;
  const authChecking = isAuthChecking.value;
  const overlayMode = authOverlayMode.value;
  const loginCelebrating = isLoginCelebrating.value;
  return {
    authChecking,
    authed,
    hideSearchBox: isSettingsOpen.value || !authed || authChecking || loginCelebrating,
    overlayMode,
    showAuthCheckingOverlay: authChecking,
    showFriends: authed && !authChecking && !loginCelebrating,
    showOverlayDim: overlayMode !== 'oobe' && (authChecking || overlayMode !== null),
    showOverlayEffects: authChecking || overlayMode !== null,
    useAuthBackground: !authed,
    showLoginWelcome: loginCelebrating,
  };
});

const updateTitleBarBackground = () => {
  const titleBar = document.getElementById('titlebar');
  if (!titleBar) return;
  titleBar.classList.toggle('bg-vrc-background', isSettingsOpen.value);
  titleBar.classList.toggle('auth-background', uiState.value.useAuthBackground);
  titleBar.classList.toggle('backdrop-blur-md', uiState.value.showOverlayEffects);
  titleBar.classList.toggle('bg-black/40', uiState.value.showOverlayDim);
};

const handleAuthOverlayMode = (mode: 'oobe' | 'auth' | null) => {
  authOverlayMode.value = mode;
};

const handleLogoutFromTitle = async () => {
  try {
    await logout();
  } catch (error) {
    console.error(error);
  } finally {
    friendsViewRef.value?.closeSettings();
    searchQuery.value = '';
    handleLogout();
  }
};

const handleSearchShortcut = (event: KeyboardEvent) => {
  const isModifier = event.ctrlKey || event.metaKey;
  if (!isModifier) return;
  const key = event.key.toLowerCase();
  if (key !== 'f' && key !== 'k') return;
  event.preventDefault();
  if (isSettingsOpen.value) {
    friendsViewRef.value?.focusSettingsSearch();
    return;
  }
  if (!uiState.value.authed || uiState.value.authChecking) return;
  titleBarRef.value?.focusSearch();
};

onMounted(async () => {
  authListener.value = await listen<AuthEvent>('vrc:auth', (event) => {
    applyAuthEvent(event.payload);
    if (event.payload.type === 'loggedOut') {
      handleLogout();
    }
  });
  window.addEventListener('keydown', handleSearchShortcut);
  updateTitleBarBackground();

  try {
    const restored = await restoreSession();
    if (restored) {
      setCurrentUser(restored);
    }
  } catch (error) {
    console.error(error);
  } finally {
    isAuthChecking.value = false;
  }
});

onBeforeUnmount(() => {
  authListener.value?.();
  authListener.value = null;
  window.removeEventListener('keydown', handleSearchShortcut);
});

watch(uiState, () => {
  updateTitleBarBackground();
});
</script>

<template>
  <main
      id="a"
      class="bg-vrc-background flex flex-col h-full overflow-hidden pt-12 relative w-full"
      :class="uiState.useAuthBackground ? 'auth-background' : ''"
  >
    <Transition name="hover-overlay">
      <div
          v-if="overlayRgb"
          :key="hoverOverlayKey"
          class="absolute inset-0 pointer-events-none z-0"
          :style="hoverOverlayStyle"
      ></div>
    </Transition>
    <Teleport to="#titlebar">
      <TitleBar
          ref="titleBarRef"
          v-model:query="searchQuery"
          :hide-search-box="uiState.hideSearchBox"
          @open-settings="handleOpenSettings"
          @open-friend-settings="handleOpenFriendSettings"
      />
    </Teleport>

    <div class="flex flex-1 flex-col items-center min-h-0 overflow-hidden relative z-10">
      <FriendsView
          v-if="uiState.showFriends"
          ref="friendsViewRef"
          :search-query="searchQuery"
          @hover-color="handleHoverColor"
          @settings-opened="isSettingsOpen = true"
          @settings-closed="isSettingsOpen = false"
          @logout="handleLogoutFromTitle"
      />

      <Teleport to="body">
        <div
            v-if="uiState.showAuthCheckingOverlay"
            class="backdrop-blur-md fixed flex inset-0 items-center justify-center pointer-events-none px-6 py-10 z-[50]"
        >
          <div class="flex flex-col gap-3 items-center pointer-events-auto text-vrc-text">
            <div
                class="animate-spin border-2 border-t-vrc-highlight/80 border-vrc-highlight/30 h-8 rounded-full w-8"></div>
            <p class="text-sm">{{ t('auth.sessionChecking') }}</p>
          </div>
        </div>
      </Teleport>

      <LoginWelcomeOverlay :show="uiState.showLoginWelcome" />

      <Oobe
          :is-auth-checking="isAuthChecking"
          @overlay-mode="handleAuthOverlayMode"
      />
    </div>
  </main>
</template>

<style scoped>
:global(.auth-background) {
  background-image: radial-gradient(circle at 18% 18%, rgba(255, 190, 214, 0.7), transparent 45%),
  radial-gradient(circle at 82% 20%, rgba(255, 178, 130, 0.6), transparent 45%),
  radial-gradient(circle at 20% 82%, rgba(255, 160, 200, 0.5), transparent 50%),
  radial-gradient(circle at 85% 78%, rgba(255, 140, 96, 0.45), transparent 50%),
  linear-gradient(135deg, rgb(255, 236, 246), rgb(255, 206, 186), rgb(255, 178, 150));
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
