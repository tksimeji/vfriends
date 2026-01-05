import {computed, Ref, ref} from 'vue';
import type {VRChat} from '../vrchat.ts';

const currentUser: Ref<VRChat.CurrentUser | null> = ref(null);

export const useAuthSession = () => {
  const isAuthenticated = computed(() => Boolean(currentUser.value));

  const setCurrentUser = (user: VRChat.CurrentUser | null) => {
    currentUser.value = user;
  };

  const clearCurrentUser = () => {
    currentUser.value = null;
  };

  return {
    currentUser,
    isAuthenticated,
    setCurrentUser,
    clearCurrentUser,
  };
};
