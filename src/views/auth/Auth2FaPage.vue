<script setup lang="ts">
import {KeyRoundIcon, LogInIcon} from 'lucide-vue-next';
import {computed} from 'vue';
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

const hint = computed(() => {
  const method = selectedMethod.value || props.methods[0];
  if (method === 'totp') return '認証アプリのコードを入力してください。';
  if (method === 'emailOtp') return 'メールで届いたコードを入力してください。';
  if (method === 'otp') return 'リカバリーコードを入力してください。';
  return '';
});

const labels: Record<TwoFactorMethod, string> = {
  totp: '認証アプリ',
  emailOtp: 'メール',
  otp: 'リカバリー',
};

const onCodeInput = (event: Event) => {
  code.value = (event.target as HTMLInputElement).value;
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="font-semibold text-lg text-vrc-highlight">2FA確認</h2>
    <p class="text-sm text-vrc-text">2FAコードを入力してください。</p>

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
          placeholder="2FAコード"
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
        戻る
      </button>
      <VrcButton :disabled="props.isSubmitting" type="button" @click="emit('submit')">
        <LogInIcon/>
        送信
      </VrcButton>
    </div>
  </div>
</template>

