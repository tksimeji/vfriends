<script setup lang="ts">
const props = defineProps<{
  error?: string;
  success?: string;
}>();
</script>

<template>
  <p v-if="props.error" class="text-sm text-red-600">{{ props.error }}</p>
  <p v-else-if="props.success" class="text-sm text-green-600">{{ props.success }}</p>
</template>
