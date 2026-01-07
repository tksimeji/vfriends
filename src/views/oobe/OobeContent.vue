<script setup lang="ts">
import {computed} from 'vue';
import {useI18n} from 'vue-i18n';
import heartUrl from '../../assets/FluentHeartSuit3d.png';
import tksimejiUrl from '../../assets/tksimeji.png';
import DiscordJoinButton from '../../components/DiscordJoinButton.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcSelect from '../../components/VrcSelect.vue';
import {type LocaleKey, setLocale} from '../../i18n';

const emit = defineEmits<{
  (e: 'complete'): void;
}>();

const {t, locale} = useI18n();
const languageOptions = computed(() => [
  {value: 'ja', label: t('settings.languageOptions.ja')},
  {value: 'en', label: t('settings.languageOptions.en')},
]);

const handleLanguageChange = (value: string) => {
  setLocale(value as LocaleKey);
};

</script>

<template>
  <div class="flex flex-col items-center h-[min(92vh,560px)] w-[min(94vw,640px)]">
    <div class="flex flex-col items-center gap-10 h-full justify-center px-4 py-8">
      <div class="flex flex-col items-center relative">
        <div class="oobe-hearts select-none" aria-hidden="true">
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
          <img class="oobe-heart" alt="Heart" :src="heartUrl" :draggable="false"/>
        </div>
        <div class="oobe-bubble select-none space-y-3">
          <Transition name="oobe-text" mode="out-in">
            <div :key="locale">
              <p class="font-semibold text-lg text-white">{{ t('oobe.thanksTitle') }}</p>
              <p class="text-sm text-white">{{ t('oobe.thanksDescription') }}</p>
            </div>
          </Transition>
          <VrcSelect
              :model-value="locale"
              :options="languageOptions"
              @update:model-value="handleLanguageChange"
          />
          <Transition name="oobe-text" mode="out-in">
            <p :key="locale" class="text-sm text-white">
              {{ t('oobe.discordPrompt') }}
            </p>
          </Transition>
          <div class="flex justify-center">
            <DiscordJoinButton/>
          </div>
        </div>
        <img
            class="h-24 mt-6 rounded-full select-none w-24"
            alt="tksimeji's Icon"
            :src="tksimejiUrl"
            :draggable="false"
        />
      </div>

      <VrcButton
          class="bg-[#cccfff]! border-[#b6b2ff]! px-10 py-3 text-base text-[#3b356d]! hover:bg-[#dbd6ff]! hover:border-[#a8a2ff]! hover:text-[#2f2b5a]!"
          type="button"
          @click="emit('complete')"
      >
        <Transition name="oobe-text" mode="out-in">
          <span :key="locale">{{ t('oobe.actionStart') }}</span>
        </Transition>
      </VrcButton>
    </div>
  </div>
</template>

<style scoped>
.oobe-hearts {
  position: absolute;
  inset: 0;
  pointer-events: none;
  overflow: hidden;
}

.oobe-heart {
  position: absolute;
  bottom: -10%;
  left: calc(var(--x) * 1%);
  width: calc(var(--size) * 1px);
  height: calc(var(--size) * 1px);
  object-fit: contain;
  animation: oobe-float 3.6s ease-in-out infinite;
  animation-delay: calc(var(--delay) * 1s);
  opacity: 0;
  pointer-events: none;
  user-select: none;
}

.oobe-heart:nth-child(1) {
  --x: 15;
  --size: 14;
  --delay: 0.2;
}

.oobe-heart:nth-child(2) {
  --x: 30;
  --size: 18;
  --delay: 0.8;
}

.oobe-heart:nth-child(3) {
  --x: 45;
  --size: 12;
  --delay: 1.2;
}

.oobe-heart:nth-child(4) {
  --x: 60;
  --size: 20;
  --delay: 0.4;
}

.oobe-heart:nth-child(5) {
  --x: 75;
  --size: 16;
  --delay: 1.6;
}

.oobe-heart:nth-child(6) {
  --x: 20;
  --size: 10;
  --delay: 2.0;
}

.oobe-heart:nth-child(7) {
  --x: 35;
  --size: 15;
  --delay: 2.6;
}

.oobe-heart:nth-child(8) {
  --x: 55;
  --size: 11;
  --delay: 3.0;
}

.oobe-heart:nth-child(9) {
  --x: 70;
  --size: 17;
  --delay: 2.2;
}

.oobe-heart:nth-child(10) {
  --x: 85;
  --size: 13;
  --delay: 1.0;
}

@keyframes oobe-float {
  0% {
    transform: translate(-50%, 20px) scale(0.9);
    opacity: 0;
  }
  20% {
    opacity: 0.9;
  }
  60% {
    opacity: 0.8;
  }
  100% {
    transform: translate(-50%, -140px) scale(1.1);
    opacity: 0;
  }
}

.oobe-bubble {
  position: relative;
  border-radius: 16px;
  background: #cccfff;
  border: none;
  box-shadow: none;
  padding: 14px 16px;
}

.oobe-bubble::after {
  content: '';
  position: absolute;
  left: 50%;
  bottom: -8px;
  width: 16px;
  height: 16px;
  background: #cccfff;
  border: none;
  transform: translateX(-50%) rotate(45deg);
}

.oobe-text-enter-active,
.oobe-text-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.oobe-text-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.oobe-text-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.oobe-text-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.oobe-text-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>