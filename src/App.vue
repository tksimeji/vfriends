<script setup lang="ts">
import {computed, ref} from 'vue';
import type {AuthUser} from './composables/useAuthFlow.ts';
import AuthModal from './views/auth/AuthModal.vue';
import FriendsView from './views/friends/FriendsView.vue';
import './style.css';
import 'vue-final-modal/style.css';

const authedUser = ref<AuthUser | null>(null);
const isAuthenticated = computed(() => Boolean(authedUser.value));

const handleLoginSuccess = (user: AuthUser | null) => {
  authedUser.value = user;
};
</script>

<template>
  <main
      id="a"
      class="relative flex min-h-screen w-full flex-col items-center bg-vrc-background"
      :class="isAuthenticated ? '' : 'auth-background'"
  >
    <FriendsView/>

    <div
        v-if="!isAuthenticated"
        class="absolute inset-0 flex items-center justify-center bg-black/40 px-6 py-10 backdrop-blur-md"
    >
      <AuthModal @login-success="handleLoginSuccess"/>
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
</style>
