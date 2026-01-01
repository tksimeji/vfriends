<script setup lang="ts">
const props = defineProps<{
  error?: string;
  success?: string;
}>();
</script>

<template>
  <p v-if="props.error" class="text-red-600 text-sm">{{ props.error }}</p>
  <p v-else-if="props.success" class="text-green-600 text-sm">{{ props.success }}</p>
</template>

