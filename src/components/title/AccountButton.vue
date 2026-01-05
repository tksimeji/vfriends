<script setup lang="ts">
import {useAuthSession} from '../../composables/useAuthSession.ts';
import VrcAvatar from '../VrcAvatar.vue';

const {currentUser} = useAuthSession();

const emit = defineEmits<{
  (e: 'open-settings'): void;
}>();

</script>

<template>
  <button
      v-if="currentUser"
      type="button"
      class="duration-150 flex items-center justify-center p-0.5 rounded-full text-vrc-text transition
       active:scale-90 hover:scale-95 hover:text-vrc-highlight"
      data-tauri-drag-region="false"
      @click="emit('open-settings')"
  >
    <VrcAvatar :user="currentUser" :size="32"/>
  </button>
</template>
