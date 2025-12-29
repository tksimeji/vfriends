<script setup lang="ts">
import {InfoIcon, KeyRoundIcon, LogInIcon, UserRoundIcon} from 'lucide-vue-next';
import VrcButton from '../../components/VrcButton.vue';
import VrcInput from '../../components/VrcInput.vue';
import AuthStatusMessage from './AuthStatusMessage.vue';

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

const onUsernameInput = (event: Event) => {
  username.value = (event.target as HTMLInputElement).value;
};

const onPasswordInput = (event: Event) => {
  password.value = (event.target as HTMLInputElement).value;
};
</script>

<template>
  <div class="flex flex-col gap-4">
    <img src="../../assets/LogInToVRChat.png" alt="Log in to VRChat"/>

    <p class="font-semibold text-red-600">VRChat copyrights of VRChat Inc. and this app is not affiliated with VRChat Inc.</p>

    <div class="flex flex-col gap-2">
      <VrcInput
          placeholder="ユーザー名"
          :value="username"
          :disabled="props.isSubmitting"
          @input="onUsernameInput"
      >
        <UserRoundIcon/>
      </VrcInput>
      <VrcInput
          type="password"
          placeholder="パスワード"
          :value="password"
          :disabled="props.isSubmitting"
          @input="onPasswordInput"
      >
        <KeyRoundIcon/>
      </VrcInput>
    </div>

    <div class="bg-vrc-highlight/70 flex gap-1 outline-1 outline-offset-1 outline-vrc-highlight/80 p-2 rounded-md text-white">
      <InfoIcon :size="16"/>
      <p>ログイン情報は VRChat APIの利用にのみ使用され，その他のサーバーに送信されることはありません．</p>
    </div>

    <AuthStatusMessage :error="props.errorMessage" :success="props.successMessage"/>

    <VrcButton :disabled="props.isSubmitting" type="button" @click="emit('submit')">
      <LogInIcon/>
      ログイン
    </VrcButton>
  </div>
</template>
