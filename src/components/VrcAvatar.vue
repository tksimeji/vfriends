<script setup lang="ts">
import {computed, useAttrs} from 'vue';
import {resolveAvatarUrl} from '../composables/useAvatarUrl';
import {VRChat} from '../vrchat.ts';

const props = withDefaults(
    defineProps<{
      user: VRChat.LimitedUserFriend | VRChat.CurrentUser;
      size?: number;
    }>(),
    {
      size: 24
    },
);

const attrs = useAttrs();

const avatarUrl = computed(() => resolveAvatarUrl(props.user));
const alt = computed(() => `${props.user.displayName}'s Icon`);
</script>

<template>
  <img
      v-if="avatarUrl"
      v-bind="attrs"
      class="object-cover rounded-full select-none"
      :src="avatarUrl"
      :alt="alt"
      :draggable="false"
      :style="{height: `${props.size}px`, width: `${props.size}px`}"
  />
</template>
