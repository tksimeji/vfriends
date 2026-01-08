<script setup lang="ts">
import {openUrl} from '@tauri-apps/plugin-opener';
import {useI18n} from 'vue-i18n';
import VrcButton from './VrcButton.vue';

const BOOTH_URL = 'https://tksimeji.booth.pm/items/7844126';

const {t} = useI18n();

const handleClick = async () => {
  const message = t('settings.about.shareText', {url: BOOTH_URL});
  const tweetUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}`;
  try {
    await openUrl(tweetUrl);
  } catch (error) {
    console.error(error);
  }
};
</script>

<template>
  <VrcButton class="bg-[#1da1f2]! h-10 text-white" @click="handleClick">
    {{ t('settings.about.shareButton') }}
    <img
        class="select-none size-6"
        src="../assets/TwitterWhite.png"
        alt="Twitter"
        :draggable="false"
    />
  </VrcButton>
</template>