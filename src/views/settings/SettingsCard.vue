<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>

<template>
  <div class="bg-vrc-background-secondary flex flex-col gap-2 px-6 py-4 rounded-xl text-vrc-text">
    <p class="font-bold text-white text-xl">{{ props.title }}</p>
    <slot/>
  </div>
</template>