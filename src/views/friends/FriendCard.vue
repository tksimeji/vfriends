<script setup lang="ts">
import {MapPinIcon, VolumeOffIcon} from 'lucide-vue-next';
import {computed, onMounted, ref, toRef, watch} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcStatus from '../../components/VrcStatus.vue';
import {resolveFriendAvatarUrl} from '../../composables/useAvatarUrl';
import {useDominantColor} from '../../composables/useDominantColor';
import {useFriendStatus} from '../../composables/useFriendStatus';
import {useFriendWorld} from '../../composables/useFriendWorld';
import {fetchFriendSettings} from '../../invokes';
import {VRChat} from '../../vrchat.ts';

type HoverPayload = {
  id: string;
  rgb: [number, number, number] | null;
  active: boolean;
};

const CARD_BORDER_FALLBACK = 'transparent';
const DEFAULT_PANEL_BG = 'rgba(12, 14, 20, 0.55)';
const TEXT_SHIELD_LAYER = 'linear-gradient(180deg, rgba(12, 14, 20, 0.35), rgba(12, 14, 20, 0.85))';

const props = withDefaults(defineProps<{
  friend: VRChat.LimitedUserFriend;
}>(), {});

const emit = defineEmits<{
  (e: 'open-settings', friend: VRChat.LimitedUserFriend): void;
  (e: 'hover', payload: HoverPayload): void;
}>();

const notificationsEnabled = ref(true);
const isHovered = ref(false);
const friendRef = toRef(props, 'friend');

const {t} = useI18n();
const {isOffline, lastOnline} = useFriendStatus(friendRef);
const {locationLabel, worldImageUrl} = useFriendWorld(friendRef);
const {overlayStyle, rgb} = useDominantColor(friendRef);

const avatarUrl = computed(() => resolveFriendAvatarUrl(props.friend));
const lastOnlineLabel = computed(() =>
    lastOnline.value ? t('friends.lastOnline', {value: lastOnline.value}) : '',
);
const showLocation = computed(() =>
    !isOffline.value && locationLabel.value,
);
const showWorldImage = computed(() => Boolean(worldImageUrl.value));
const showInlineStatus = computed(() => !isOffline.value);
const borderGradient = computed(() => {
  if (!rgb.value) return '';
  const [r, g, b] = rgb.value;
  return `linear-gradient(135deg, rgba(${r}, ${g}, ${b}, 0.9), rgba(${r}, ${g}, ${b}, 0.2))`;
});
const cardStyle = computed(() => ({
  '--card-border-gradient': borderGradient.value || CARD_BORDER_FALLBACK,
  '--card-border-opacity': isHovered.value && borderGradient.value ? '1' : '0',
  borderColor: isHovered.value && borderGradient.value ? 'transparent' : undefined,
}));
const panelStyle = computed(() => {
  const base = overlayStyle.value ?? {};
  if (!showWorldImage.value) return base;
  const gradient = typeof base.backgroundImage === 'string' ? base.backgroundImage : '';
  const [r, g, b] = rgb.value ?? [];
  const tint = rgb.value
      ? `linear-gradient(180deg, rgba(${r}, ${g}, ${b}, 0.5), rgba(${r}, ${g}, ${b}, 0.2))`
      : '';
  const imageLayer = `url("${worldImageUrl.value}")`;
  const layers = [
    TEXT_SHIELD_LAYER,
    tint,
    gradient,
    imageLayer,
  ].filter(Boolean);
  return {
    ...base,
    backgroundColor: rgb.value
        ? `rgba(${r}, ${g}, ${b}, 0.2)`
        : DEFAULT_PANEL_BG,
    backgroundImage: layers.join(', '),
    backgroundPosition: 'center',
    backgroundSize: 'cover',
  };
});

const refreshNotificationStatus = async () => {
  try {
    const map = await fetchFriendSettings();
    notificationsEnabled.value = map?.[props.friend.id]?.enabled !== false;
  } catch (error) {
    console.error(error);
    notificationsEnabled.value = true;
  }
};

const emitHover = (active: boolean) => {
  emit('hover', {id: props.friend.id, rgb: rgb.value, active});
};

const openSettings = () => {
  emit('open-settings', props.friend);
};

const handleMouseEnter = () => {
  isHovered.value = true;
  emitHover(true);
};

const handleMouseLeave = () => {
  isHovered.value = false;
  emitHover(false);
};

watch(
    () => props.friend.id,
    () => {
      void refreshNotificationStatus();
    },
);

watch(rgb, () => {
  if (isHovered.value) {
    emitHover(true);
  }
});

onMounted(() => {
  void refreshNotificationStatus();
});
</script>

<template>
  <article
      class="bg-vrc-background-secondary border-3 border-vrc-background-secondary cursor-pointer duration-150 flex flex-col friend-card group overflow-hidden rounded-2xl select-none transition-colors"
      :style="cardStyle"
      @click="openSettings"
      @mouseenter="handleMouseEnter"
      @mouseleave="handleMouseLeave"
  >
    <img
        alt=""
        class="aspect-video duration-200 ease-out object-cover rounded-t-xl transition-transform group-hover:scale-105"
        loading="lazy"
        :src="avatarUrl"
        :class="isOffline ? 'opacity-70' : ''"
    />
    <div
        class="bg-linear-to-b flex flex-col from-vrc-background gap-2 p-2 to-vrc-background-secondary"
        :style="panelStyle"
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
      <VrcStatus
          v-if="!showInlineStatus"
          class="gap-1"
          label-class="text-vrc-highlight/60"
          :user="props.friend"
          :size="12"
      />
      <div v-if="showLocation" class="flex gap-3 items-center p-2">
        <div class="flex flex-1 flex-col gap-1 min-w-0">
          <VrcStatus
              v-if="showInlineStatus"
              class="gap-1"
              label-class="text-vrc-highlight/60"
              :user="props.friend"
              :size="12"
          />
          <div class="flex gap-2 items-center min-w-0">
            <MapPinIcon class="shrink-0 text-vrc-highlight/70" :size="14"/>
            <span class="font-semibold text-sm text-vrc-text/90 truncate">
              {{ locationLabel }}
            </span>
          </div>
        </div>
      </div>
      <span v-if="lastOnlineLabel" class="text-xs text-vrc-text">
        {{ lastOnlineLabel }}
      </span>
    </div>
  </article>
</template>

<style scoped>
.friend-card {
  position: relative;
  --card-border-gradient: transparent;
  --card-border-opacity: 0;
}

.friend-card::before {
  content: '';
  position: absolute;
  inset: 0;
  padding: 3px;
  border-radius: inherit;
  background: var(--card-border-gradient, transparent);
  opacity: var(--card-border-opacity, 0);
  pointer-events: none;
  transition: opacity 0.2s ease;
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
}
</style>
