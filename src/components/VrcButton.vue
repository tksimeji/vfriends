<script setup lang="ts">
import {computed, useAttrs} from 'vue';

type Size = 'md' | 'sm';

const props = defineProps<{
  size?: Size;
  loading?: boolean;
}>();

const attrs = useAttrs();
const passThroughAttrs = computed(() => {
  const {class: _class, style: _style, ...rest} = attrs;
  return rest;
});

const classes = computed(() => {
  const base =
    'bg-vrc-button/80 border-2 border-vrc-highlight/20 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-vrc-highlight/40 font-semibold gap-2 inline-flex items-center justify-center rounded-md text-vrc-highlight tracking-[0.24em] transition disabled:cursor-not-allowed disabled:opacity-60 hover:bg-vrc-button hover:border-vrc-highlight/80 hover:text-vrc-highlight';
  const sizes: Record<Size, string> = {
    md: 'px-4 py-2 text-xs',
    sm: 'px-3 py-1.5 text-[11px]',
  };
  return [base, sizes[props.size ?? 'md']].join(' ');
});

const isDisabled = computed(() => Boolean(attrs.disabled) || props.loading);
</script>

<template>
  <button
      v-bind="passThroughAttrs"
      :class="classes"
      :disabled="isDisabled"
      :aria-busy="props.loading || undefined"
  >
    <span
        v-if="props.loading"
        class="animate-spin border-2 border-current border-t-transparent h-4 rounded-full w-4"
    ></span>
    <slot/>
  </button>
</template>
