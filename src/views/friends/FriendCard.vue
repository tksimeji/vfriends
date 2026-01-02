<script setup lang="ts">
import {VolumeOffIcon} from 'lucide-vue-next';
import {computed, onMounted, ref, watch} from 'vue';
import {useNotificationPreferences} from '../../composables/notifications/useNotificationPreferences';
import {useDominantColor} from '../../composables/useDominantColor';
import StatusBadge from '../../components/StatusBadge.vue';
import {VRChat} from '../../vrchat.ts';

const props = withDefaults(defineProps<{
  friend: VRChat.LimitedUserFriend;
}>(), {});

const emit = defineEmits<{
  (e: 'open-settings'): void;
  (e: 'hover', payload: { id: string; rgb: [number, number, number] | null; active: boolean }): void;
}>();

const {isEnabled, load} = useNotificationPreferences();

onMounted(() => {
  void load();
});

const isOffline = computed(() => VRChat.isOffline(props.friend));
const avatarUrl = computed(() => VRChat.avatarUrl(props.friend));
const colorSource = computed(() => props.friend);
const {overlayStyle, rgb} = useDominantColor(colorSource);
const isHovered = ref(false);

const lastOnline = computed(() => VRChat.formatLastOnline(props.friend));

const notificationsEnabled = computed(() => isEnabled(props.friend.id));

const emitHover = (active: boolean) => {
  emit('hover', {id: props.friend.id, rgb: rgb.value, active});
};

watch(rgb, () => {
  if (isHovered.value) {
    emitHover(true);
  }
});
</script>

<template>
  <article
      class="bg-vrc-background-secondary border-3 border-vrc-background-secondary cursor-pointer duration-150 flex flex-col overflow-y-hidden rounded-2xl select-none transition-colors hover:bg-vrc-highlight/10 hover:border-vrc-highlight/70"
      @click="emit('open-settings')"
      @mouseenter="() => { isHovered = true; emitHover(true); }"
      @mouseleave="() => { isHovered = false; emitHover(false); }"
  >
    <img
        :src="avatarUrl"
        alt=""
        class="aspect-video object-cover rounded-t-xl"
        :class="isOffline ? 'opacity-70' : ''"
        loading="lazy"
    />
    <div
        class="bg-linear-to-b flex flex-col from-vrc-background gap-2 p-2 to-vrc-background-secondary"
        :style="overlayStyle"
    >
      <div class="flex gap-2 items-center justify-between">
        <div class="flex gap-2 items-center">
          <p class="font-bold text-vrc-friend text-xl">{{ props.friend.displayName }}</p>
          <VolumeOffIcon
              v-if="!notificationsEnabled"
              class="text-red-600"
              :size="16"
          />
        </div>
      </div>
      <StatusBadge
          :friend="props.friend"
          class="gap-1"
          label-class="text-vrc-highlight/60"
          :size="12"
      />
      <span v-if="lastOnline" class="text-vrc-text text-xs">
        最終オンライン：{{ lastOnline }}
      </span>
    </div>
  </article>
</template>

