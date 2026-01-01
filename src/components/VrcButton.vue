<script setup lang="ts">
import {computed, useAttrs} from 'vue';

type Variant = 'primary' | 'secondary' | 'ghost';
type Size = 'md' | 'sm';

const props = defineProps<{
  variant?: Variant;
  size?: Size;
  loading?: boolean;
}>();

const attrs = useAttrs();

const classes = computed(() => {
  const base =
    'inline-flex items-center justify-center gap-2 rounded-md border-2 px-4 py-2 text-xs font-semibold uppercase tracking-[0.24em] transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-vrc-highlight/40 disabled:cursor-not-allowed disabled:opacity-60';
  const sizes: Record<Size, string> = {
    md: 'px-4 py-2 text-xs',
    sm: 'px-3 py-1.5 text-[11px]',
  };
  const variants: Record<Variant, string> = {
    primary:
      'border-vrc-highlight/40 bg-vrc-button/80 text-vrc-text shadow-[0_16px_36px_-26px_rgba(0,0,0,0.7)] hover:border-vrc-highlight/80 hover:bg-vrc-highlight/10 hover:text-vrc-highlight',
    secondary:
      'border-vrc-highlight/25 bg-transparent text-vrc-subtext hover:border-vrc-highlight/60 hover:text-vrc-highlight',
    ghost: 'border-transparent text-vrc-subtext hover:text-vrc-highlight',
  };
  return [
    base,
    sizes[props.size ?? 'md'],
    variants[props.variant ?? 'primary'],
  ].join(' ');
});

const isDisabled = computed(() => Boolean(attrs.disabled) || props.loading);
</script>

<template>
  <button
      v-bind="attrs"
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

