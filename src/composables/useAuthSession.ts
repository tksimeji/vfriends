import {computed, Ref, ref} from 'vue';
import type {VRChat} from '../vrchat.ts';

export type AuthAction = 'credentials' | 'twoFactor';
export type AuthEvent =
  | { type: 'started'; action: AuthAction }
  | { type: 'twoFactorRequired'; methods?: string[]; message?: string }
  | { type: 'success'; user?: VRChat.CurrentUser }
  | { type: 'failure'; message: string; code?: string }
  | { type: 'loggedOut' };

const currentUser: Ref<VRChat.CurrentUser | null> = ref(null);
const isLoginCelebrating = ref(false);
let celebrationTimer: number | null = null;

export const useAuthSession = () => {
  const isAuthenticated = computed(() => Boolean(currentUser.value));

  const setCurrentUser = (user: VRChat.CurrentUser | null) => {
    currentUser.value = user;
  };

  const clearCurrentUser = () => {
    currentUser.value = null;
  };

  const applyAuthEvent = (event: AuthEvent) => {
    if (event.type === 'success') {
      if (event.user) setCurrentUser(event.user);
      return;
    }
    if (event.type === 'loggedOut') {
      clearCurrentUser();
    }
  };

  const startLoginCelebration = (durationMs = 1600) => {
    isLoginCelebrating.value = true;
    if (celebrationTimer !== null) {
      window.clearTimeout(celebrationTimer);
    }
    celebrationTimer = window.setTimeout(() => {
      isLoginCelebrating.value = false;
      celebrationTimer = null;
    }, durationMs);
  };

  return {
    currentUser,
    isAuthenticated,
    setCurrentUser,
    clearCurrentUser,
    applyAuthEvent,
    isLoginCelebrating,
    startLoginCelebration,
  };
};
