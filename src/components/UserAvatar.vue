<script setup lang="ts">
import {computed} from 'vue';

const props = withDefaults(
  defineProps<{
    src?: string;
    name?: string;
    sizeClass?: string;
    borderClass?: string;
    fallbackClass?: string;
    alt?: string;
  }>(),
  {
    src: '',
    name: '',
    sizeClass: 'size-6',
    borderClass: 'border border-vrc-highlight/30',
    fallbackClass: 'text-[10px] font-semibold',
    alt: '',
  },
);

const initial = computed(() => {
  const name = props.name?.trim();
  return name ? name.charAt(0).toUpperCase() : '?';
});
</script>

<template>
  <img
      v-if="props.src"
      :src="props.src"
      :alt="props.alt"
      class="object-cover rounded-full"
      :class="[props.sizeClass, props.borderClass]"
  />
  <span
      v-else
      class="bg-vrc-background flex items-center justify-center rounded-full"
      :class="[props.sizeClass, props.borderClass, props.fallbackClass]"
  >
    {{ initial }}
  </span>
</template>

