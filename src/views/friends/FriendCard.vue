<script setup lang="ts">
import {Temporal} from '@js-temporal/polyfill';
import {computed} from 'vue';
import {VRChat} from '../../vrchat.ts';

const props = withDefaults(defineProps<{
  friend: VRChat.LimitedUserFriend;
}>(), {});

const isOffline = computed(() => VRChat.isOffline(props.friend));
const statusKey = computed(() => props.friend.status.toLowerCase());
const displayStatus = computed(() => (isOffline.value ? 'offline' : statusKey.value));
const statusColor = computed(() => {
  switch (displayStatus.value) {
    case 'join me':
      return 'bg-vrc-join-me';
    case 'active':
      return 'bg-vrc-online';
    case 'ask me':
      return 'bg-vrc-ask-me';
    case 'busy':
      return 'bg-vrc-do-not-disturb';
    default:
      return 'bg-black';
  }
});
const statusLabel = computed(() => {
  switch (displayStatus.value) {
    case 'join me':
      return 'だれでもおいで';
    case 'active':
      return 'オンライン';
    case 'ask me':
      return 'きいてみてね';
    case 'busy':
      return '取り込み中';
    default:
      return 'オフライン';
  }
});
const avatarUrl = computed(
    () =>
        props.friend.profilePicOverrideThumbnail ||
        props.friend.currentAvatarThumbnailImageUrl ||
        props.friend.imageUrl,
);

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
</script>

<template>
  <article
      class="flex select-none flex-col overflow-y-hidden rounded-2xl border-3 border-vrc-background-secondary bg-vrc-background-secondary transition-colors duration-150 hover:border-vrc-highlight/70 hover:bg-vrc-highlight/10"
  >
    <img
        :src="avatarUrl"
        alt=""
        class="aspect-video rounded-t-xl object-cover"
        :class="isOffline ? 'opacity-70' : ''"
        loading="lazy"
    />
    <div class="bg-linear-to-b flex flex-col gap-2 p-2 from-vrc-background to-vrc-background-secondary">
      <p class="font-bold text-vrc-friend text-xl">{{ props.friend.displayName }}</p>
      <div class="flex items-center gap-1">
        <div class="size-3 rounded-full" :class="statusColor"/>
        <span class="text-vrc-highlight/60">{{ statusLabel }}</span>
      </div>
      <span v-if="lastOnline" class="text-xs text-vrc-text">
        最終オンライン：{{ lastOnline }}
      </span>
    </div>
  </article>
</template>
