<script setup lang="ts">
import {computed} from 'vue';
import TitleBarSearch from './TitleBarSearch.vue';
import AccountMenu from './AccountMenu.vue';
import type {AuthUser} from '../composables/useAuthFlow';
import type {VRChat} from '../vrchat.ts';

const props = defineProps<{
  query: string;
  authedUser: AuthUser | null;
  hideSearch?: boolean;
  suggestions?: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'update:query', value: string): void;
  (e: 'open-settings'): void;
  (e: 'open-friend-settings', friendId: string): void;
  (e: 'logout'): void;
}>();

const isAuthed = computed(() => Boolean(props.authedUser));

const openSettings = () => {
  emit('open-settings');
};

const handleLogout = () => {
  emit('logout');
};

</script>

<template>
  <div
      class="flex gap-3 h-full items-center overflow-visible px-3 w-full"
      :style="{paddingRight: 'var(--tauri-frame-controls-width, 0px)'}"
      data-tauri-drag-region
  >
    <div class="flex gap-2 items-center min-w-0 shrink-0 text-vrc-text text-xs" data-tauri-drag-region>
      <span class="font-semibold text-vrc-friend tracking-wide">vfriends</span>
    </div>

    <div class="flex flex-1 justify-center min-w-0" data-tauri-drag-region>
      <TitleBarSearch
          v-if="isAuthed && !hideSearch"
          :model-value="query"
          :suggestions="suggestions"
          @update:model-value="(value) => emit('update:query', value)"
          @select="(friendId) => emit('open-friend-settings', friendId)"
      />
    </div>

    <div class="flex gap-2 items-center shrink-0">
      <AccountMenu
          v-if="authedUser"
          :user="authedUser"
          @open-settings="openSettings"
          @logout="handleLogout"
      />
    </div>
  </div>
</template>

