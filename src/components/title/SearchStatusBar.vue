<script setup lang="ts">
import {computed} from 'vue';
import {resolveUserStatus} from '../../composables/useFriendStatus';
import type {VRChat} from '../../vrchat.ts';

const STATUS_ORDER: VRChat.UserStatus[] = ['join me', 'active', 'ask me', 'busy', 'offline'];
const STATUS_COLORS: Record<VRChat.UserStatus, string> = {
  'join me': 'bg-vrc-join-me',
  'active': 'bg-vrc-online',
  'ask me': 'bg-vrc-ask-me',
  'busy': 'bg-vrc-do-not-disturb',
  'offline': 'bg-white',
};

const props = defineProps<{
  focused: boolean;
  friends: VRChat.LimitedUserFriend[];
  query: string;
}>();

const normalizedQuery = computed(() => props.query.trim().toLowerCase());
const visibleFriends = computed(() => {
  const normalized = normalizedQuery.value;
  if (!normalized) return props.friends;
  return props.friends
      .filter((friend) => friend.displayName.toLowerCase().includes(normalized));
});
const segments = computed(() => {
  const counts = STATUS_ORDER.reduce<Record<VRChat.UserStatus, number>>((acc, status) => {
    acc[status] = 0;
    return acc;
  }, {} as Record<VRChat.UserStatus, number>);
  for (const friend of visibleFriends.value) {
    const status = resolveUserStatus(friend);
    counts[status] += 1;
  }
  const total = visibleFriends.value.length;
  if (!total) return [];
  return STATUS_ORDER
      .map((status) => ({
        status,
        colorClass: STATUS_COLORS[status],
        count: counts[status],
        ratio: counts[status] / total,
      }))
      .filter((segment) => segment.count > 0);
});
const hasSegments = computed(() => segments.value.length > 0);
</script>

<template>
  <div class="absolute bottom-0 h-0.5 inset-x-0 overflow-hidden rounded-b-md">
    <div
        v-if="!props.focused"
        class="bg-vrc-highlight/40 h-full w-full"
    />
    <div
        v-else-if="!hasSegments"
        class="bg-vrc-highlight/15 h-full w-full"
    />
    <div
        v-else
        class="flex h-full w-full"
    >
      <span
          v-for="segment in segments"
          class="h-full status-segment"
          :key="segment.status"
          :class="segment.colorClass"
          :style="{width: `${segment.ratio * 100}%`}"
      />
    </div>
  </div>
</template>

<style scoped>
.status-segment {
  transition: width 0.25s ease;
}
</style>