<script setup lang="ts">
import VrcButton from '../../components/VrcButton.vue';

import type {VRChat} from '../../vrchat.ts';
import {useI18n} from 'vue-i18n';

const props = withDefaults(defineProps<{
  user: VRChat.CurrentUser;
  isSubmitting?: boolean;
}>(), {
  isSubmitting: false,
});

const emit = defineEmits<{
  (event: 'logout'): void;
  (event: 'close'): void;
}>();

const {t} = useI18n();
</script>

<template>
  <section class="flex flex-col gap-4">
    <div class="bg-vrc-background border-2 border-vrc-highlight/20 p-6 rounded-xl text-vrc-text">
      <h2 class="font-semibold text-vrc-highlight text-xl">{{ t('auth.loginSuccess') }}</h2>
      <p class="mt-2 text-sm text-vrc-text/70">{{ t('auth.loginSuccessDetail') }}</p>

      <div class="flex flex-col gap-2 mt-4">
        <div class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">{{ t('auth.displayName') }}</span>
          <span class="font-semibold">{{ props.user.displayName }}</span>
        </div>
        <div v-if="props.user.id" class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">{{ t('auth.userId') }}</span>
          <span class="font-mono text-sm">{{ props.user.id }}</span>
        </div>
        <div v-if="props.user.username" class="flex gap-2 items-center">
          <span class="text-vrc-text/60 text-xs uppercase">{{ t('auth.usernameLabel') }}</span>
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
        {{ t('auth.success.close') }}
      </button>
      <VrcButton :disabled="props.isSubmitting" @click="emit('logout')">
        {{ t('auth.success.logout') }}
      </VrcButton>
    </div>
  </section>
</template>

