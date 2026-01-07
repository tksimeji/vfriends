<script setup lang="ts">
import {BellIcon, BellOffIcon, MoreHorizontalIcon, XIcon} from 'lucide-vue-next';
import {computed, nextTick, ref, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from '../../components/VrcButton.vue';
import {resolveFriendAvatarUrl} from '../../composables/useAvatarUrl';
import type {VRChat} from '../../vrchat.ts';

const props = defineProps<{
  hasSelection: boolean;
  selectedCount: number;
  selectedFriends: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'enable'): void;
  (e: 'disable'): void;
  (e: 'clear'): void;
}>();

const {t} = useI18n();

const showActions = computed(() => props.hasSelection);
const selectionLabel = computed(() => t('friends.bulkToast.selectionCount', {count: props.selectedCount}));
const avatarLimit = 4;
const visibleFriends = computed(() => props.selectedFriends.slice(0, avatarLimit));
const hiddenCount = computed(() =>
    Math.max(0, props.selectedFriends.length - visibleFriends.value.length),
);

const containerRef = ref<HTMLDivElement | null>(null);

const animateSizeChange = async () => {
  const element = containerRef.value;
  if (!element) return;
  const first = element.getBoundingClientRect();
  await nextTick();
  const last = element.getBoundingClientRect();
  if (!first.width || !first.height || !last.width || !last.height) return;
  const dx = first.left - last.left;
  const dy = first.top - last.top;
  const sx = first.width / last.width;
  const sy = first.height / last.height;
  if (!Number.isFinite(sx) || !Number.isFinite(sy)) return;
  element.animate(
      [
        {transform: `translate(${dx}px, ${dy}px) scale(${sx}, ${sy})`},
        {transform: 'translate(0, 0) scale(1, 1)'},
      ],
      {duration: 200, easing: 'ease-out'},
  );
};

watch(
    () => props.selectedCount,
    () => {
      void animateSizeChange();
    },
    {flush: 'pre'},
);
</script>

<template>
  <Transition name="selection-actions">
    <div
        v-if="showActions"
        class="absolute bottom-6 flex gap-2 inset-x-0 justify-center px-4 z-20"
        data-selection-actions
        @pointerdown.stop
    >
      <div
          ref="containerRef"
          class="backdrop-blur-md bg-black/85 border border-white/10 flex gap-3 items-center px-4 py-2 rounded-full shadow-[0_16px_40px_rgba(0,0,0,0.45)]"
      >
        <div class="flex gap-3 items-center">
          <div class="flex gap-2 items-center">
            <TransitionGroup class="-space-x-2 flex items-center" name="avatar-stack" tag="div">
              <img
                  v-for="friend in visibleFriends"
                  :key="friend.id"
                  class="bg-black/60 border border-white/20 h-7 object-cover rounded-full shadow-sm w-7"
                  :src="resolveFriendAvatarUrl(friend)"
                  alt=""
              />
              <div
                  v-if="hiddenCount > 0"
                  key="overflow"
                  class="bg-white/10 border border-white/20 flex h-7 items-center justify-center rounded-full text-vrc-text/60 text-xs w-7"
              >
                <MoreHorizontalIcon :size="14"/>
              </div>
            </TransitionGroup>
            <span class="select-none text-xs text-vrc-text/60">
              {{ selectionLabel }}
            </span>
          </div>
          <div class="flex gap-2 items-center">
            <VrcButton
                class="bg-emerald-500/15! border-emerald-400/40! text-emerald-100! hover:bg-emerald-500/25! hover:border-emerald-300/60! hover:text-emerald-50!"
                @click="emit('enable')"
            >
              <span class="flex gap-2 items-center">
                <BellIcon :size="14"/>
                {{ t('friends.contextMenu.enableNotifications') }}
              </span>
            </VrcButton>
            <VrcButton
                class="bg-rose-500/15! border-rose-400/40! text-rose-100! hover:bg-rose-500/25! hover:border-rose-300/60! hover:text-rose-50!"
                @click="emit('disable')"
            >
              <span class="flex gap-2 items-center">
                <BellOffIcon :size="14"/>
                {{ t('friends.contextMenu.disableNotifications') }}
              </span>
            </VrcButton>
            <button
                class="bg-white/5 border border-white/15 flex h-8 items-center justify-center rounded-full text-vrc-text/70 transition-colors w-8 hover:bg-white/10 hover:text-vrc-text"
                type="button"
                @click="emit('clear')"
            >
              <XIcon :size="14"/>
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.selection-actions-enter-active,
.selection-actions-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.selection-actions-enter-from,
.selection-actions-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.selection-actions-enter-to,
.selection-actions-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.avatar-stack-enter-active,
.avatar-stack-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.avatar-stack-enter-from,
.avatar-stack-leave-to {
  opacity: 0;
  transform: translateY(6px) scale(0.9);
}

.avatar-stack-enter-to,
.avatar-stack-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}
</style>