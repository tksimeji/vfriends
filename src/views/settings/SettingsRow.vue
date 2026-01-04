<script setup lang="ts">
defineSlots<{
  description: () => any;
  control: () => any;
}>();
</script>

<template>
  <div class="flex gap-3 items-start justify-between">
    <div class="flex-1 min-w-0">
      <slot name="description"/>
    </div>
    <div class="shrink-0">
      <slot name="control"/>
    </div>
  </div>
</template>
