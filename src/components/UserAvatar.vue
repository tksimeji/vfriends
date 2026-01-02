<script setup lang="ts">
import {computed} from 'vue';
import {VRChat} from '../vrchat.ts';

const props = withDefaults(
  defineProps<{
    user: VRChat.LimitedUserFriend | VRChat.CurrentUser;
    size?: number;
    borderClass?: string;
    fallbackClass?: string;
    alt?: string;
  }>(),
  {
    size: 24,
    borderClass: 'border border-vrc-highlight/30',
    fallbackClass: 'font-semibold text-[10px]',
    alt: '',
  },
);

const avatarUrl = computed(() => VRChat.resolveAvatarUrl(props.user));
const displayName = computed(() => props.user.displayName ?? '');
const altText = computed(() => props.alt || displayName.value);
const initial = computed(() => {
  const name = displayName.value.trim();
  return name ? name.charAt(0).toUpperCase() : '?';
});
</script>

<template>
  <img
      v-if="avatarUrl"
      :src="avatarUrl"
      :alt="altText"
      class="object-cover rounded-full"
      :class="[borderClass]"
      :style="{height: `${props.size}px`, width: `${props.size}px`}"
  />
  <span
      v-else
      class="bg-vrc-background flex items-center justify-center rounded-full"
      :class="[borderClass, fallbackClass]"
      :style="{height: `${props.size}px`, width: `${props.size}px`}"
  >
    {{ initial }}
  </span>
</template>

