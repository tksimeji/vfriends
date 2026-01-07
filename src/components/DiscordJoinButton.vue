<script setup lang="ts">
import {openUrl} from '@tauri-apps/plugin-opener';
import {useI18n} from 'vue-i18n';
import discordUrl from '../assets/Discord.png';
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
  <VrcButton class="bg-[#5865f2]! text-white!" @click="handleClick">
    {{ t('oobe.discordButton') }}
    <img class="select-none" :width="24" alt="Discord" :src="discordUrl" :draggable="false"/>
  </VrcButton>
</template>