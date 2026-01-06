<script setup lang="ts">
import {computed, onMounted, ref, watch} from 'vue';
import {useAuthSession} from '../../composables/useAuthSession';
import AuthModal from '../auth/AuthModal.vue';
import OobeContent from './OobeContent.vue';

const props = defineProps<{
  isAuthChecking: boolean;
}>();
const emit = defineEmits<{
  (e: 'overlay-mode', value: 'oobe' | 'auth' | null): void;
}>();

const OOBE_KEY = 'oobe.completed';
const isOobeComplete = ref(false);
const {isAuthenticated} = useAuthSession();

const showOobe = computed(
    () => !props.isAuthChecking && !isAuthenticated.value && !isOobeComplete.value,
);
const overlayMode = computed<'oobe' | 'auth' | null>(() => {
  if (showOobe.value) return 'oobe';
  if (!props.isAuthChecking && !isAuthenticated.value) return 'auth';
  return null;
});

const handleOobeComplete = () => {
  isOobeComplete.value = true;
  if (typeof window !== 'undefined') {
    window.localStorage.setItem(OOBE_KEY, 'true');
  }
};

watch(
    overlayMode,
    (mode) => {
      emit('overlay-mode', mode);
    },
    {immediate: true},
);

onMounted(() => {
  if (typeof window !== 'undefined') {
    isOobeComplete.value = window.localStorage.getItem(OOBE_KEY) === 'true';
  }
});
</script>

<template>
  <Teleport to="body">
    <div
        v-if="overlayMode"
        class="fixed flex inset-x-0 bottom-0 items-center justify-center px-6 py-10 top-12 z-[50]"
    >
      <div class="oobe-warp">
        <Transition name="oobe-auth" mode="out-in">
          <div :key="overlayMode">
            <OobeContent v-if="overlayMode === 'oobe'" @complete="handleOobeComplete"/>
            <AuthModal v-else/>
          </div>
        </Transition>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.oobe-auth-enter-active,
.oobe-auth-leave-active {
  animation-duration: 0.7s;
  animation-fill-mode: both;
  animation-timing-function: cubic-bezier(0.2, 0.9, 0.2, 1);
  will-change: transform, opacity, filter;
}

.oobe-auth-enter-from {
  opacity: 0;
}

.oobe-auth-enter-to {
  opacity: 1;
}

.oobe-auth-leave-from {
  opacity: 1;
}

.oobe-auth-leave-to {
  opacity: 0;
}

.oobe-auth-enter-active {
  animation-name: oobe-warp-in;
}

.oobe-auth-leave-active {
  animation-name: oobe-warp-out;
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.oobe-warp {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  perspective: 1000px;
}

.oobe-warp > div {
  transform-origin: center;
  transform-style: preserve-3d;
}

@keyframes oobe-warp-in {
  0% {
    opacity: 0;
    transform: scale(0.96);
    filter: blur(6px) saturate(1.2);
  }
  60% {
    opacity: 1;
    transform: scale(1.01);
    filter: blur(1px) saturate(1.08);
  }
  100% {
    opacity: 1;
    transform: scale(1);
    filter: blur(0) saturate(1);
  }
}

@keyframes oobe-warp-out {
  0% {
    opacity: 1;
    transform: scale(1);
    filter: blur(0) saturate(1);
  }
  100% {
    opacity: 0;
    transform: scale(0.94);
    filter: blur(8px) saturate(1.2);
  }
}
</style>