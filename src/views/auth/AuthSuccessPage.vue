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
    <div class="rounded-xl border-2 border-vrc-highlight/20 bg-vrc-background p-6 text-vrc-text">
      <h2 class="text-xl font-semibold text-vrc-highlight">ログイン成功</h2>
      <p class="mt-2 text-sm text-vrc-text/70">VRChatにログインしました。</p>

      <div class="mt-4 flex flex-col gap-2">
        <div class="flex items-center gap-2">
          <span class="text-xs uppercase text-vrc-text/60">Display Name</span>
          <span class="font-semibold">{{ props.user.displayName }}</span>
        </div>
        <div v-if="props.user.id" class="flex items-center gap-2">
          <span class="text-xs uppercase text-vrc-text/60">User ID</span>
          <span class="font-mono text-sm">{{ props.user.id }}</span>
        </div>
        <div v-if="props.user.username" class="flex items-center gap-2">
          <span class="text-xs uppercase text-vrc-text/60">Username</span>
          <span class="text-sm">{{ props.user.username }}</span>
        </div>
      </div>
    </div>

    <div class="flex items-center gap-2">
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
