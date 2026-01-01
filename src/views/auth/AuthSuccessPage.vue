<script setup lang="ts">
import VrcButton from '../../components/VrcButton.vue';

const props = withDefaults(defineProps<{
  user: {
    id: string;
    displayName: string;
    username?: string | null;
  };
  isSubmitting?: boolean;
}>(), {
  isSubmitting: false,
});

const emit = defineEmits<{
  (event: 'logout'): void;
  (event: 'close'): void;
}>();
</script>

<template>
  <section class="flex flex-col gap-4">
    <div class="bg-vrc-background border-2 border-vrc-highlight/20 p-6 rounded-xl text-vrc-text">
      <h2 class="font-semibold text-vrc-highlight text-xl">ログイン成功</h2>
      <p class="mt-2 text-sm text-vrc-text/70">VRChatにログインしました。</p>

      <div class="flex flex-col gap-2 mt-4">
        <div class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">Display Name</span>
          <span class="font-semibold">{{ props.user.displayName }}</span>
        </div>
        <div v-if="props.user.id" class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">User ID</span>
          <span class="font-mono text-sm">{{ props.user.id }}</span>
        </div>
        <div v-if="props.user.username" class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">Username</span>
          <span class="text-sm">{{ props.user.username }}</span>
        </div>
      </div>
    </div>

    <div class="flex gap-2 items-center">
      <button
          type="button"
          class="px-2 text-sm text-vrc-text/70 hover:text-vrc-highlight"
          :disabled="props.isSubmitting"
          @click="emit('close')"
      >
        閉じる
      </button>
      <VrcButton :disabled="props.isSubmitting" @click="emit('logout')">ログアウト</VrcButton>
    </div>
  </section>
</template>

