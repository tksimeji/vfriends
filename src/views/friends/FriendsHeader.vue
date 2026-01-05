<script setup lang="ts">
import {computed} from 'vue';
import {SearchIcon, SettingsIcon, XIcon} from 'lucide-vue-next';
import {resolveCurrentUserAvatarUrl} from '../../composables/useAvatarUrl';
import {VRChat} from '../../vrchat.ts';
import type {FriendsStatusMessage} from './types';
import {useI18n} from 'vue-i18n';

const props = defineProps<{
  query: string;
  searchActive: boolean;
  filteredCount: number;
  totalCount: number;
  isLoading: boolean;
  statusMessage: FriendsStatusMessage | null;
  authedUser: VRChat.CurrentUser | null;
}>();

const emit = defineEmits<{
  (e: 'update:query', value: string): void;
  (e: 'open-settings'): void;
}>();

const accountAvatarUrl = computed(() => {
  const user = props.authedUser;
  if (!user) return '';
  return resolveCurrentUserAvatarUrl(user);
});

const accountInitial = computed(() => {
  const name = props.authedUser?.displayName ?? '';
  return name ? name.trim().charAt(0).toUpperCase() : '?';
});
const {t} = useI18n();
const countLabel = computed(() =>
  props.searchActive
    ? t('friends.countFiltered', {
      filtered: props.filteredCount,
      total: props.totalCount,
    })
    : t('friends.count', {count: props.totalCount}),
);

const clearSearch = () => {
  emit('update:query', '');
};

const openSettings = () => {
  emit('open-settings');
};

</script>

<template>
  <div class="backdrop-blur bg-vrc-background/95 border-b border-vrc-highlight/15 py-2 shrink-0">
    <div class="flex flex-wrap gap-3 items-center justify-between">
      <div class="flex flex-col gap-1">
        <p
            v-if="statusMessage"
            class="text-sm"
            :class="statusMessage.tone === 'error' ? 'text-red-300' : 'text-vrc-text/70'"
        >
          {{ statusMessage.text }}
        </p>
        <div class="flex gap-3 items-center text-vrc-text/60 text-xs">
          <span>{{ countLabel }}</span>
          <span class="hiddensm:inline">{{ t('friends.clickCardHint') }}</span>
          <span v-if="isLoading" class="text-vrc-highlight/70">{{ t('friends.updating') }}</span>
        </div>
      </div>
      <div class="flex items-center max-w-sm mx-auto relative w-full">
        <SearchIcon class="absolute left-3 pointer-events-none text-vrc-text/50" :size="14" />
        <input
            :value="query"
            type="text"
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 outline-none pl-8 pr-8 py-2 rounded-md text-vrc-text text-xs w-full focus:border-vrc-highlight/70"
            :placeholder="t('friends.searchPlaceholder')"
            @input="emit('update:query', ($event.target as HTMLInputElement).value)"
        />
        <button
            v-if="searchActive"
            type="button"
            class="absolute right-2 text-vrc-text/50 hover:text-vrc-text"
            @click="clearSearch"
        >
          <XIcon :size="14" />
        </button>
      </div>
      <div class="flex gap-2 items-center ml-auto">
        <button
            v-if="authedUser"
            type="button"
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex gap-2 items-center px-2 py-1 rounded-full text-vrc-text text-xs transition hover:border-vrc-highlight/60"
            @click="openSettings"
        >
          <span
              v-if="!accountAvatarUrl"
              class="bg-vrc-background border border-vrc-highlight/30 flex font-semibold items-center justify-center rounded-full size-6 text-[10px]"
          >
            {{ accountInitial }}
          </span>
          <img
              v-else
              :src="accountAvatarUrl"
              alt=""
              class="border border-vrc-highlight/30 object-cover rounded-full size-6"
          />
          <span class="hidden max-w-[120px] truncate sm:inline">{{ authedUser.displayName }}</span>
        </button>

        <button
            type="button"
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex h-9 items-center justify-center rounded-md text-vrc-text transition w-9 hover:border-vrc-highlight/60 hover:text-vrc-highlight"
            @click="openSettings"
        >
          <SettingsIcon :size="16" />
        </button>
      </div>
    </div>
  </div>
</template>

