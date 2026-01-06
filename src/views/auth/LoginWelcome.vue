<script setup lang="ts">
import {useI18n} from 'vue-i18n';
import VrcAvatar from '../../components/VrcAvatar.vue';
import {useAuthSession} from '../../composables/useAuthSession';

const {t} = useI18n();
const {currentUser} = useAuthSession();
</script>

<template>
  <div class="flex flex-col items-center">
    <VrcAvatar
        v-if="currentUser"
        :size="120"
        :user="currentUser"
        class="ring-4 ring-white/40"
    />
    <p class="font-semibold mt-6 text-2xl text-white">{{ t('auth.welcomeTitle') }}</p>
    <p class="text-sm text-white/80">
      {{ t('auth.welcomeSubtitle', {name: currentUser?.displayName ?? t('auth.vrchatUserFallback')}) }}</p>
  </div>
</template>