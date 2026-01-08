<script setup lang="ts">
import {openUrl} from '@tauri-apps/plugin-opener';
import {useI18n} from 'vue-i18n';
import VrcButton from './VrcButton.vue';

const DISCORD_URL = 'https://discord.gg/Dus2WmpvUE';

const {t} = useI18n();

const handleClick = async () => {
  try {
    await openUrl(DISCORD_URL);
  } catch (error) {
    console.error(error);
  }
};
</script>

<template>
  <VrcButton class="bg-[#5865f2]! h-10 text-white!" @click="handleClick">
    {{ t('oobe.discordButton') }}
    <img
        class="select-none w-6"
        src="../assets/Discord.png"
        alt="Discord"
        :draggable="false"
    />
  </VrcButton>
</template>