<script setup lang="ts">
import {KeyRoundIcon, LogInIcon} from 'lucide-vue-next';
import {computed} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from '../../components/VrcButton.vue';
import VrcInput from '../../components/VrcInput.vue';
import AuthStatusMessage from './AuthStatusMessage.vue';

type TwoFactorMethod = 'totp' | 'emailOtp' | 'otp';

const props = withDefaults(defineProps<{
  methods?: TwoFactorMethod[];
  isSubmitting?: boolean;
  errorMessage?: string;
  successMessage?: string;
}>(), {
  methods: () => [],
  isSubmitting: false,
  errorMessage: '',
  successMessage: '',
});

const code = defineModel<string>('code', {default: ''});
const selectedMethod = defineModel<TwoFactorMethod | ''>('selectedMethod', {default: ''});

const emit = defineEmits<{
  (event: 'submit'): void;
  (event: 'back'): void;
}>();

const {t} = useI18n();

const hint = computed(() => {
  const method = selectedMethod.value || props.methods[0];
  if (method === 'totp') return t('auth.twoFactorMethod.totpHelp');
  if (method === 'emailOtp') return t('auth.twoFactorMethod.emailHelp');
  if (method === 'otp') return t('auth.twoFactorMethod.recoveryHelp');
  return '';
});

const labels: Record<TwoFactorMethod, string> = {
  totp: t('auth.twoFactorMethod.totpLabel'),
  emailOtp: t('auth.twoFactorMethod.emailLabel'),
  otp: t('auth.twoFactorMethod.recoveryLabel'),
};

const onCodeInput = (event: Event) => {
  code.value = (event.target as HTMLInputElement).value;
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="font-semibold text-lg text-vrc-highlight">{{ t('auth.twoFactorTitle') }}</h2>
    <p class="text-sm text-vrc-text">{{ t('auth.twoFactorPrompt') }}</p>

    <div class="flex flex-col gap-2">
      <select
          v-if="props.methods.length > 1"
          :value="selectedMethod"
          :disabled="props.isSubmitting"
          class="bg-vrc-button/70 border-2 border-vrc-highlight/20 outline-none p-2 rounded-md text-vrc-highlight"
          @change="selectedMethod = ($event.target as HTMLSelectElement).value as TwoFactorMethod"
      >
        <option v-for="method in props.methods" :key="method" :value="method">
          {{ labels[method] }}
        </option>
      </select>

      <VrcInput
          :placeholder="t('auth.twoFactorCodePlaceholder')"
          :value="code"
          :disabled="props.isSubmitting"
          @input="onCodeInput"
      >
        <KeyRoundIcon/>
      </VrcInput>

      <p v-if="hint" class="text-vrc-text/70 text-xs">{{ hint }}</p>
    </div>

    <AuthStatusMessage :error="props.errorMessage" :success="props.successMessage"/>

    <div class="flex gap-2 items-center">
      <button
          type="button"
          class="px-2 text-sm text-vrc-text/70 hover:text-vrc-highlight"
          :disabled="props.isSubmitting"
          @click="emit('back')"
      >
        {{ t('auth.back') }}
      </button>
      <VrcButton :disabled="props.isSubmitting" type="button" @click="emit('submit')">
        <LogInIcon/>
        {{ t('auth.submit') }}
      </VrcButton>
    </div>
  </div>
</template>

