<script setup lang="ts">
import {computed, useAttrs} from 'vue';
import {useI18n} from 'vue-i18n';
import {resolveUserStatus} from '../composables/useFriendStatus';
import {VRChat} from '../vrchat.ts';

defineOptions({inheritAttrs: false});

const props = withDefaults(defineProps<{
  user: VRChat.LimitedUserFriend;
  size?: number;
  labelClass?: string;
  showLabel?: boolean;
}>(), {
  user: undefined,
  statusKey: '',
  showLabel: true,
  size: 12,
  labelClass: 'text-vrc-highlight/60 text-xs',
});

const attrs = useAttrs();
const {t} = useI18n();

const status = computed(() => {
  switch (resolveUserStatus(props.user)) {
    case 'join me':
      return {label: 'userStatus.joinMe', colorClass: 'bg-vrc-join-me'};
    case 'active':
      return {label: 'userStatus.online', colorClass: 'bg-vrc-online'};
    case 'ask me':
      return {label: 'userStatus.askMe', colorClass: 'bg-vrc-ask-me'};
    case 'busy':
      return {label: 'userStatus.doNotDisturb', colorClass: 'bg-vrc-do-not-disturb'};
    case 'offline':
      return {label: 'userStatus.offline', colorClass: 'bg-black'};
  }
});
</script>

<template>
  <span
      class="gap-2 inline-flex items-center select-none"
      v-bind="attrs"
  >
    <span
        class="rounded-full"
        :class="status.colorClass"
        :style="{height: `${props.size}px`, width: `${props.size}px`}"
    />
    <span v-if="props.showLabel" :class="props.labelClass">
      {{ t(status.label) }}
    </span>
  </span>
</template>
