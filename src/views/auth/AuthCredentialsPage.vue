<script setup lang="ts">
import {InfoIcon, KeyRoundIcon, LogInIcon, UserRoundIcon} from 'lucide-vue-next';
import VrcButton from '../../components/VrcButton.vue';
import VrcInput from '../../components/VrcInput.vue';
import AuthStatusMessage from './AuthStatusMessage.vue';
import {useI18n} from 'vue-i18n';

const props = withDefaults(defineProps<{
  isSubmitting?: boolean;
  errorMessage?: string;
  successMessage?: string;
}>(), {
  isSubmitting: false,
  errorMessage: '',
  successMessage: '',
});

const username = defineModel<string>('username', {default: ''});
const password = defineModel<string>('password', {default: ''});

const emit = defineEmits<{
  (event: 'submit'): void;
}>();

const {t} = useI18n();

const onUsernameInput = (event: Event) => {
  username.value = (event.target as HTMLInputElement).value;
};

const onPasswordInput = (event: Event) => {
  password.value = (event.target as HTMLInputElement).value;
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <img :alt="t('auth.loginSubtitle')" src="../../assets/LogInToVRChat.png"/>

    <p class="font-semibold text-red-600">{{ t('auth.copyrightNotice') }}</p>

    <div class="flex flex-col gap-2">
      <VrcInput
          :placeholder="t('auth.usernamePlaceholder')"
          :value="username"
          :disabled="props.isSubmitting"
          @input="onUsernameInput"
      >
        <UserRoundIcon/>
      </VrcInput>
      <VrcInput
          type="password"
          :placeholder="t('auth.passwordPlaceholder')"
          :value="password"
          :disabled="props.isSubmitting"
          @input="onPasswordInput"
      >
        <KeyRoundIcon/>
      </VrcInput>
    </div>

    <div class="bg-vrc-highlight/70 flex gap-1 outline-1 outline-offset-1 outline-vrc-highlight/80 p-2 rounded-md text-white">
      <InfoIcon :size="16"/>
      <p>{{ t('auth.disclaimer') }}</p>
    </div>

    <AuthStatusMessage :error="props.errorMessage" :success="props.successMessage"/>

    <VrcButton :disabled="props.isSubmitting" type="button" @click="emit('submit')">
      <LogInIcon/>
      {{ t('auth.login') }}
    </VrcButton>
  </div>
</template>
