<script setup lang="ts">
import UserAvatar from '../UserAvatar.vue';
import type {VRChat} from '../../vrchat.ts';

const props = defineProps<{
  user: VRChat.CurrentUser;
}>();

const emit = defineEmits<{
  (e: 'open-settings'): void;
}>();

</script>

<template>
  <button
      type="button"
      class="duration-150 flex items-center justify-center p-0.5 rounded-full text-vrc-text transition active:scale-95 hover:scale-90 hover:text-vrc-highlight"
      data-tauri-drag-region="false"
      @click="emit('open-settings')"
  >
    <UserAvatar
        :user="user"
        :size="32"
        border-class="border-0"
        fallback-class="font-semibold text-[10px]"
    />
  </button>
</template>
