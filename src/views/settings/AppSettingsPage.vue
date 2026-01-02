<script setup lang="ts">
import UserAvatar from '../../components/UserAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcFilePicker from '../../components/VrcFilePicker.vue';
import VrcInput from '../../components/VrcInput.vue';
import type {VRChat} from '../../vrchat.ts';
import SettingsCard from './SettingsCard.vue';

const props = defineProps<{
  authedUser: VRChat.CurrentUser | null;
  messageTemplate: string;
  previewText: string;
  soundLabel: string;
  errorMessage: string | null;
}>();

const emit = defineEmits<{
  (e: 'update:messageTemplate', value: string): void;
  (e: 'clear-sound'): void;
  (e: 'logout'): void;
  (e: 'preview-sound'): void;
  (e: 'select-sound', file: File | null): void;
}>();
</script>

<template>
  <div class="flex flex-col gap-4 p-5">
    <SettingsCard title="アカウント設定">
      <div class="flex flex-col gap-2 items-center">
        <UserAvatar
            v-if="authedUser"
            :user="authedUser"
            :size="128"
            fallback-class="font-semibold text-[12px]"
        />
        <div class="flex flex-col items-center text-vrc-text">
          <p class="font-bold text-2xl">{{ authedUser?.displayName ?? 'VRChat User' }}</p>
          <p v-if="authedUser?.username" class="text-sm text-vrc-text/60">
            {{ authedUser.username }}
          </p>
        </div>
      </div>
      <div class="flex justify-center">
        <VrcButton
            size="sm"
            class="bg-red-500/10! border-red-500/40! text-red-200! hover:bg-red-500/20! hover:border-red-400! hover:text-red-100!"
            @click="emit('logout')"
        >
          ログアウトする
        </VrcButton>
      </div>
    </SettingsCard>

    <SettingsCard title="通知設定">
      <p class="text-sm text-vrc-text/70">
        フレンドに個別の設定を行わない場合、この設定が適用されます。
      </p>

      <VrcInput
          label="通知メッセージ"
          :value="messageTemplate"
          placeholder="{name} is online"
          @input="emit('update:messageTemplate', ($event.target as HTMLInputElement).value)"
      />
      <p class="text-vrc-text/70 text-xs">プレビュー: {{ previewText }}</p>

      <VrcFilePicker
          label="通知音（ファイル選択）"
          :value="soundLabel"
          helper="クリックして音声ファイルを選択"
          :clearable="true"
          accept=".mp3,.wav,.ogg,.flac,.m4a,audio/*"
          @select="(file) => emit('select-sound', file)"
          @clear="emit('clear-sound')"
      />
      <div class="flex flex-wrap gap-2">
        <VrcButton size="sm" @click="emit('preview-sound')">テスト再生</VrcButton>
      </div>
      <div class="flex gap-3 items-center">
        <span v-if="errorMessage" class="text-red-300 text-xs">{{ errorMessage }}</span>
      </div>
    </SettingsCard>
  </div>
</template>
