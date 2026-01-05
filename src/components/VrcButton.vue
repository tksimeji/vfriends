<script setup lang="ts">
import {computed, useAttrs} from 'vue';

const props = defineProps<{
  loading?: boolean;
}>();

const attrs = useAttrs();

const isDisabled = computed(() => Boolean(attrs.disabled) || props.loading);
</script>

<template>
  <button
      v-bind="attrs"
      class="bg-vrc-button/80 border-2 border-vrc-highlight/20 font-semibold gap-2 inline-flex items-center justify-center p-4 py-2 rounded-md select-none text-vrc-highlight text-xs transition
       disabled:cursor-not-allowed disabled:opacity-60
       focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-vrc-highlight/40
       hover:bg-vrc-button hover:border-vrc-highlight/80 hover:text-vrc-highlight"
      :disabled="isDisabled"
      :aria-busy="props.loading || undefined"
  >
    <span
        v-if="props.loading"
        class="animate-spin border-2 border-current border-t-transparent h-4 rounded-full w-4"
    />
    <slot/>
  </button>
</template>