<script setup lang="ts">
import {onBeforeUnmount, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    event: { enabled: boolean; count: number; id: number } | null;
    durationMs?: number;
  }>(),
  {
    durationMs: 2000,
  },
);

const {t} = useI18n();
const visibleMessage = ref<string | null>(null);
const isVisible = ref(false);
let toastTimer: number | null = null;

const clearTimer = () => {
  if (toastTimer) {
    window.clearTimeout(toastTimer);
    toastTimer = null;
  }
};

const showToast = (event: { enabled: boolean; count: number }) => {
  const messageKey = event.enabled
    ? 'friends.bulkToast.enabled'
    : 'friends.bulkToast.disabled';
  visibleMessage.value = t(messageKey, {count: event.count});
  isVisible.value = true;
  clearTimer();
  toastTimer = window.setTimeout(() => {
    isVisible.value = false;
    visibleMessage.value = null;
    toastTimer = null;
  }, props.durationMs);
};

watch(
  () => props.event?.id,
  () => {
    if (!props.event) return;
    showToast(props.event);
  },
  {immediate: true},
);

onBeforeUnmount(() => {
  clearTimer();
});
</script>

<template>
  <Transition name="action-toast">
    <div v-if="isVisible && visibleMessage" class="absolute bottom-6 flex inset-x-0 justify-center px-4 z-20">
      <div
          class="backdrop-blur-md bg-vrc-highlight/80 border border-white/10 px-4 py-2 rounded-full select-none shadow-[0_12px_30px_rgba(0,0,0,0.4)] text-sm text-white"
      >
        {{ visibleMessage }}
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.action-toast-enter-active,
.action-toast-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.action-toast-enter-from,
.action-toast-leave-to {
  opacity: 0;
  transform: translateY(8px) scale(0.98);
}

.action-toast-enter-to,
.action-toast-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}
</style>