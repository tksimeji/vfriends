<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>

<template>
  <div class="bg-vrc-background-secondary border border-vrc-text/40 flex flex-col gap-4 px-6 py-4 rounded-xl select-none text-vrc-text">
    <p class="font-bold text-white text-xl">{{ props.title }}</p>
    <slot/>
  </div>
</template>