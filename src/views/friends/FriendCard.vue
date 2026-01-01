<script setup lang="ts">
import {Temporal} from '@js-temporal/polyfill';
import {computed, onMounted} from 'vue';
import {VolumeOffIcon} from 'lucide-vue-next';
import {useNotificationPreferences} from '../../composables/notifications/useNotificationPreferences';
import {VRChat} from '../../vrchat.ts';

const props = withDefaults(defineProps<{
  friend: VRChat.LimitedUserFriend;
}>(), {});

const emit = defineEmits<{
  (e: 'open-settings'): void;
}>();

const {isEnabled, load} = useNotificationPreferences();

onMounted(() => {
  void load();
});

const isOffline = computed(() => VRChat.isOffline(props.friend));
const statusKey = computed(() => VRChat.statusKey(props.friend));
const statusColor = computed(() => VRChat.statusColorClass(statusKey.value));
const statusLabel = computed(() => VRChat.statusLabel(statusKey.value));
const avatarUrl = computed(() => VRChat.avatarUrl(props.friend));

const lastOnline = computed(() => {
  if (!isOffline.value) return null;

  const rawValue =
      props.friend.last_activity ??
      (props.friend as { lastActivity?: string }).lastActivity ??
      props.friend.last_login ??
      (props.friend as { lastLogin?: string }).lastLogin;
  if (!rawValue) return null;

  try {
    const lastOnline = Temporal.Instant.from(rawValue);
    const now = Temporal.Now.instant();

    const diff = now.since(lastOnline, {largestUnit: 'hour'});

    const diffMinutes = Math.floor(diff.total({unit: 'minute'}));
    if (diffMinutes < 60) {
      return `${diffMinutes}分前`;
    }

    const diffHours = Math.floor(diff.total({unit: 'hour'}));
    if (diffHours < 24) {
      return `${diffHours}時間前`;
    }

    const diffDays = Math.floor(diffHours / 24);
    return `${diffDays}日前`;
  } catch {
    return null;
  }
});

const notificationsEnabled = computed(() => isEnabled(props.friend.id));
</script>

<template>
  <article
      class="bg-vrc-background-secondary border-3 border-vrc-background-secondary cursor-pointer duration-150 flex flex-col overflow-y-hidden rounded-2xl select-none transition-colors hover:bg-vrc-highlight/10 hover:border-vrc-highlight/70"
      @click="emit('open-settings')"
  >
    <img
        :src="avatarUrl"
        alt=""
        class="aspect-video object-cover rounded-t-xl"
        :class="isOffline ? 'opacity-70' : ''"
        loading="lazy"
    />
    <div class="bg-linear-to-b flex flex-col from-vrc-background gap-2 p-2 to-vrc-background-secondary">
      <div class="flex gap-2 items-center justify-between">
        <div class="flex gap-2 items-center">
          <p class="font-bold text-vrc-friend text-xl">{{ props.friend.displayName }}</p>
          <VolumeOffIcon
              v-if="notificationsEnabled === false"
              class="text-vrc-text/50"
              :size="16"
          />
        </div>
      </div>
      <div class="flex gap-1 items-center">
        <div class="rounded-full size-3" :class="statusColor"/>
        <span class="text-vrc-highlight/60">{{ statusLabel }}</span>
      </div>
      <span v-if="lastOnline" class="text-vrc-text text-xs">
        最終オンライン：{{ lastOnline }}
      </span>
    </div>
  </article>
</template>

