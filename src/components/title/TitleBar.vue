<script setup lang="ts">
import {computed} from 'vue';
import {useAuthSession} from '../../composables/useAuthSession';
import AccountButton from './AccountButton.vue';
import SearchBox from './SearchBox.vue';

const props = defineProps<{
  query: string;
  hideSearchBox?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:query', value: string): void;
  (e: 'open-settings'): void;
  (e: 'open-friend-settings', friendId: string): void;
}>();

const {currentUser} = useAuthSession();

const openSettings = () => {
  emit('open-settings');
};
</script>

<template>
  <div
      class="flex gap-3 h-full items-center overflow-visible px-3 select-none w-full"
      :style="{paddingRight: 'var(--tauri-frame-controls-width, 0px)'}"
      :data-tauri-drag-region="true"
  >
    <div class="flex gap-2 items-center min-w-0 shrink-0" :data-tauri-drag-region="true">
      <AccountButton
          v-if="currentUser"
          :user="currentUser"
          @open-settings="openSettings"
      />
    </div>

    <div class="flex flex-1 justify-center min-w-0" :data-tauri-drag-region="true">
      <SearchBox
          v-if="Boolean(currentUser) && !props.hideSearchBox"
          :model-value="props.query"
          @update:model-value="(value) => emit('update:query', value)"
          @select="(friendId) => emit('open-friend-settings', friendId)"
      />
    </div>
  </div>
</template>