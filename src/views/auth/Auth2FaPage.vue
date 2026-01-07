<script setup lang="ts">
import {KeyRoundIcon, LogInIcon} from 'lucide-vue-next';
import {computed} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from '../../components/VrcButton.vue';
import VrcInput from '../../components/VrcInput.vue';
import VrcSelect from '../../components/VrcSelect.vue';
import AuthStatusMessage from './AuthStatusMessage.vue';

type TwoFactorMethod = 'totp' | 'emailOtp' | 'otp';

const METHOD_LABEL_KEYS: Record<TwoFactorMethod, string> = {
  totp: 'auth.twoFactorMethod.totpLabel',
  emailOtp: 'auth.twoFactorMethod.emailLabel',
  otp: 'auth.twoFactorMethod.recoveryLabel',
};

const METHOD_HELP_KEYS: Record<TwoFactorMethod, string> = {
  totp: 'auth.twoFactorMethod.totpHelp',
  emailOtp: 'auth.twoFactorMethod.emailHelp',
  otp: 'auth.twoFactorMethod.recoveryHelp',
};

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

const methodOptions = computed(() =>
  props.methods.map((method) => ({
    value: method,
    label: t(METHOD_LABEL_KEYS[method]),
  })),
);

const hint = computed(() => {
  const method = selectedMethod.value || props.methods[0];
  if (method === 'totp') return t(METHOD_HELP_KEYS.totp);
  if (method === 'emailOtp') return t(METHOD_HELP_KEYS.emailOtp);
  if (method === 'otp') return t(METHOD_HELP_KEYS.otp);
  return '';
});

const onCodeInput = (event: Event) => {
  code.value = (event.target as HTMLInputElement).value;
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="font-semibold select-none text-lg text-vrc-highlight">{{ t('auth.twoFactorTitle') }}</h2>
    <p class="select-none text-sm text-vrc-text">{{ t('auth.twoFactorPrompt') }}</p>

    <div class="flex flex-col gap-2">
      <VrcSelect
          v-if="props.methods.length > 1"
          :disabled="props.isSubmitting"
          :model-value="selectedMethod"
          :options="methodOptions"
          @update:model-value="(value) => selectedMethod = value as TwoFactorMethod"
      />

      <VrcInput
          :placeholder="t('auth.twoFactorCodePlaceholder')"
          :value="code"
          :disabled="props.isSubmitting"
          @input="onCodeInput"
      >
        <KeyRoundIcon/>
      </VrcInput>

      <p v-if="hint" class="select-none text-vrc-text text-xs">{{ hint }}</p>
    </div>

    <AuthStatusMessage :error="props.errorMessage" :success="props.successMessage"/>

    <div class="flex gap-2 items-center">
      <button
          type="button"
          class="px-2 select-none text-sm text-vrc-text hover:text-vrc-highlight"
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