<script setup lang="ts">
import {computed, useAttrs} from 'vue';
import {VRChat} from '../vrchat.ts';

defineOptions({inheritAttrs: false});

const props = withDefaults(defineProps<{
  friend?: VRChat.LimitedUserFriend;
  statusKey?: string;
  showLabel?: boolean;
  size?: number;
  labelClass?: string;
}>(), {
  friend: undefined,
  statusKey: '',
  showLabel: true,
  size: 12,
  labelClass: 'text-vrc-highlight/60 text-xs',
});

const attrs = useAttrs();

const resolvedStatusKey = computed(() => {
  if (props.friend) return VRChat.statusKey(props.friend);
  if (props.statusKey) return props.statusKey;
  return 'offline';
});

const label = computed(() => VRChat.statusLabel(resolvedStatusKey.value));
const colorClass = computed(() => VRChat.statusColorClass(resolvedStatusKey.value));
</script>

<template>
  <span class="gap-2 inline-flex items-center select-none" :class="attrs.class" :style="attrs.style">
    <span
        class="rounded-full"
        :class="colorClass"
        :style="{height: `${props.size}px`, width: `${props.size}px`}"
    />
    <span v-if="props.showLabel" :class="props.labelClass">{{ label }}</span>
  </span>
</template>
