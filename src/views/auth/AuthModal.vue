<script setup lang="ts">
import AuthCredentialsPage from './AuthCredentialsPage.vue';
import AuthSuccessPage from './AuthSuccessPage.vue';
import Auth2FaPage from './Auth2FaPage.vue';
import {useAuthFlow} from '../../composables/useAuthFlow.ts';

const {
  username,
  password,
  twoFactorCode,
  twoFactorMethods,
  selectedTwoFactorMethod,
  isSubmitting,
  errorMessage,
  successMessage,
  currentStep,
  handleCredentialsSubmit,
  handleTwoFactorSubmit,
  handleBackToCredentials,
  handleSuccessClose,
} = useAuthFlow();
</script>

<template>
  <div
      class="backdrop-blur-2xl bg-vrc-background/40 h-[min(90vh,520px)] overflow-y-auto p-8 rounded-4xl shadow-2xl ring-1 ring-white/10 w-[min(90vw,520px)]"
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
        :user="{ id: '', displayName: 'VRChat User' }"
        :is-submitting="isSubmitting"
        @close="handleSuccessClose"
    />
  </div>
</template>

<style scoped>
</style>
