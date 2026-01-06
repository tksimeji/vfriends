<script setup lang="ts">
import LoginWelcome from './LoginWelcome.vue';

const props = defineProps<{
  show: boolean;
}>();
</script>

<template>
  <Teleport to="body">
    <Transition name="login-welcome">
      <div
          v-if="props.show"
          class="fixed flex inset-x-0 bottom-0 items-center justify-center px-6 py-10 top-12 z-[55]"
      >
        <LoginWelcome/>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.login-welcome-enter-active,
.login-welcome-leave-active {
  transition: opacity 0.4s ease, transform 0.4s ease;
}

.login-welcome-enter-from {
  opacity: 0;
  transform: translateY(12px) scale(0.96);
}

.login-welcome-enter-to {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.login-welcome-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.login-welcome-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}
</style>