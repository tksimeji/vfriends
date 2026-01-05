<script setup lang="ts">
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, onBeforeUnmount, onMounted, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import TitleBar from './components/title/TitleBar.vue';
import './style.css';
import AuthModal from './views/auth/AuthModal.vue';
import FriendsView from './views/friends/FriendsView.vue';
import 'vue-final-modal/style.css';
import {useAuthSession} from './composables/useAuthSession';
import {useDominantColor} from './composables/useDominantColor';
import {logout, restoreSession} from './invokes';

const {currentUser, isAuthenticated, setCurrentUser, clearCurrentUser} = useAuthSession();
const {t} = useI18n();
const {rgb: currentUserRgb} = useDominantColor(currentUser);
const searchQuery = ref('');
const isSettingsOpen = ref(false);
const hoverColor = ref<[number, number, number] | null>(null);
const authListener = ref<UnlistenFn | null>(null);
const isAuthChecking = ref(true);
const titleBarRef = ref<{ focusSearch: () => void } | null>(null);

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
  focusSettingsSearch: () => void;
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
const updateTitleBarBackground = () => {
  const titleBar = document.getElementById('titlebar');
  if (!titleBar) return;
  if (isSettingsOpen.value) {
    titleBar.classList.add('bg-vrc-background');
  } else {
    titleBar.classList.remove('bg-vrc-background');
  }
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

const handleAuthEvent = (event: AuthEvent) => {
  if (event.type === 'success') {
    if (event.user) setCurrentUser(event.user);
    return;
  }
  if (event.type === 'loggedOut') {
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
  if (!isAuthenticated.value || isAuthChecking.value) return;
  titleBarRef.value?.focusSearch();
};

onMounted(async () => {
  authListener.value = await listen<AuthEvent>('vrc:auth', (event) => {
    handleAuthEvent(event.payload);
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

watch(isSettingsOpen, () => {
  updateTitleBarBackground();
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
          :hide-search-box="isSettingsOpen || !isAuthenticated || isAuthChecking"
          @open-settings="handleOpenSettings"
          @open-friend-settings="handleOpenFriendSettings"
      />
    </Teleport>

    <div class="flex flex-1 flex-col items-center min-h-0 mt-4 overflow-hidden relative z-10">
      <FriendsView
          v-if="isAuthenticated && !isAuthChecking"
          ref="friendsViewRef"
          :search-query="searchQuery"
          @hover-color="handleHoverColor"
          @settings-opened="isSettingsOpen = true"
          @settings-closed="isSettingsOpen = false"
          @logout="handleLogoutFromTitle"
      />

      <div
          v-if="isAuthChecking"
          class="absolute backdrop-blur-md bg-black/40 flex inset-0 items-center justify-center px-6 py-10"
      >
        <div class="flex flex-col gap-3 items-center text-vrc-text">
          <div class="animate-spin border-2 border-t-vrc-highlight/80 border-vrc-highlight/30 h-8 rounded-full w-8"></div>
          <p class="text-sm">{{ t('auth.sessionChecking') }}</p>
        </div>
      </div>

      <div
          v-else-if="!isAuthenticated"
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
