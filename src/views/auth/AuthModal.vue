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
    <Transition name="auth-step" mode="out-in">
      <div :key="currentStep" class="min-h-full">
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
    </Transition>
  </div>
</template>

<style scoped>
.auth-step-enter-active,
.auth-step-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}

.auth-step-enter-from {
  opacity: 0;
  transform: translateY(10px) scale(0.98);
}

.auth-step-enter-to {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.auth-step-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.auth-step-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}
</style>