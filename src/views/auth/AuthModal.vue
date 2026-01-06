<script setup lang="ts">
import {useI18n} from 'vue-i18n';
import {useAuthFlow} from '../../composables/useAuthFlow.ts';
import Auth2FaPage from './Auth2FaPage.vue';
import AuthCredentialsPage from './AuthCredentialsPage.vue';
import AuthSuccessPage from './AuthSuccessPage.vue';

const {
  username,
  password,
  twoFactorCode,
  twoFactorMethods,
  selectedTwoFactorMethod,
  isSubmitting,
  errorMessage,
  successMessage,
  authedUser,
  currentStep,
  handleCredentialsSubmit,
  handleTwoFactorSubmit,
  handleBackToCredentials,
  handleSuccessClose,
  handleLogout,
} = useAuthFlow();

const {t} = useI18n();
</script>

<template>
  <div
      class="backdrop-blur-sm bg-vrc-background/80 h-[min(90vh,520px)] overflow-y-auto p-8 ring-1 ring-white/10 rounded-xl shadow-2xl w-[min(90vw,520px)]"
  >
    <AuthCredentialsPage
        v-if="currentStep === 'credentials'"
        v-model:username="username"
        v-model:password="password"
        :is-submitting="isSubmitting"
        :error-message="errorMessage"
        :success-message="successMessage"
        @submit="handleCredentialsSubmit"
    />

    <Auth2FaPage
        v-else-if="currentStep === 'twoFactor'"
        :methods="twoFactorMethods"
        v-model:code="twoFactorCode"
        v-model:selected-method="selectedTwoFactorMethod"
        :is-submitting="isSubmitting"
        :error-message="errorMessage"
        :success-message="successMessage"
        @submit="handleTwoFactorSubmit"
        @back="handleBackToCredentials"
    />

    <AuthSuccessPage
        v-else
        :user="authedUser ?? { id: '', displayName: t('auth.vrchatUserFallback'), currentAvatarImageUrl: '', profilePicOverride: '', userIcon: '' }"
        :is-submitting="isSubmitting"
        @close="handleSuccessClose"
        @logout="handleLogout"
    />
  </div>
</template>

<style scoped>
</style>