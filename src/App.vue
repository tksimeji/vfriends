<script setup lang="ts">
import AuthModal from './views/auth/AuthModal.vue';
import './style.css';
import 'vue-final-modal/style.css';
</script>

<template>
  <main id="a" class="flex items-center justify-center min-h-screen px-6 py-10">
    <AuthModal/>
  </main>
</template>

<style scoped>
#a {
  background-image: url('./assets/LoginBackground.png');
  background-repeat: no-repeat;
  background-position: center;
  background-size: cover;
}
</style>
