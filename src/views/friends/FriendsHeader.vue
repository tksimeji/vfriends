<script setup lang="ts">
import {computed} from 'vue';
import {Popover, PopoverButton, PopoverPanel} from '@headlessui/vue';
import {SearchIcon, SettingsIcon, XIcon} from 'lucide-vue-next';
import VrcButton from '../../components/VrcButton.vue';
import type {AuthUser} from '../../composables/useAuthFlow';
import type {FriendsStatusMessage} from './types';

const props = defineProps<{
  query: string;
  searchActive: boolean;
  filteredCount: number;
  totalCount: number;
  isLoading: boolean;
  statusMessage: FriendsStatusMessage | null;
  authedUser: AuthUser | null;
}>();

const emit = defineEmits<{
  (e: 'update:query', value: string): void;
  (e: 'open-settings'): void;
  (e: 'logout'): void;
}>();

const accountAvatarUrl = computed(() => {
  const user = props.authedUser;
  if (!user) return '';
  return (
    user.profilePicOverrideThumbnail ||
    user.currentAvatarThumbnailImageUrl ||
    user.userIcon ||
    user.imageUrl ||
    ''
  );
});

const accountInitial = computed(() => {
  const name = props.authedUser?.displayName ?? '';
  return name ? name.trim().charAt(0).toUpperCase() : '?';
});

const clearSearch = () => {
  emit('update:query', '');
};

const openSettings = () => {
  emit('open-settings');
};

const handleLogout = () => {
  emit('logout');
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
          <span>{{ searchActive ? `${filteredCount} / ${totalCount}件` : `${totalCount}件` }}</span>
          <span class="hiddensm:inline">カードをクリックで通知設定</span>
          <span v-if="isLoading" class="text-vrc-highlight/70">更新中...</span>
        </div>
      </div>
      <div class="flex items-center max-w-sm mx-auto relative w-full">
        <SearchIcon class="absolute left-3 pointer-events-none text-vrc-text/50" :size="14" />
        <input
            :value="query"
            type="text"
            class="bg-vrc-button/80 border-2 border-vrc-highlight/20 outline-none pl-8 pr-8 py-2 rounded-md text-vrc-text text-xs w-full focus:border-vrc-highlight/70"
            placeholder="フレンド検索..."
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
        <Popover class="relative" v-if="authedUser">
          <PopoverButton
              class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex gap-2 items-center px-2 py-1 rounded-full text-vrc-text text-xs transition hover:border-vrc-highlight/60"
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
          </PopoverButton>
          <Transition
              enter="transition ease-out duration-150"
              enter-from="opacity-0 translate-y-2"
              enter-to="opacity-100 translate-y-0"
              leave="transition ease-in duration-100"
              leave-from="opacity-100 translate-y-0"
              leave-to="opacity-0 translate-y-2"
          >
            <PopoverPanel
                class="absolute bg-vrc-background-secondary border-2 border-vrc-highlight/30 mt-2 p-3 right-0 rounded-md shadow-[0_18px_30px_-24px_rgba(0,0,0,0.8)] text-vrc-text text-xs w-48"
            >
              <div class="mb-2">
                <p class="font-semibold">{{ authedUser.displayName }}</p>
                <p v-if="authedUser.username" class="text-[10px] text-vrc-text/60">
                  {{ authedUser.username }}
                </p>
              </div>
              <VrcButton size="sm" variant="secondary" @click="handleLogout">ログアウト</VrcButton>
            </PopoverPanel>
          </Transition>
        </Popover>

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

