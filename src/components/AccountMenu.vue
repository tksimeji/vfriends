<script setup lang="ts">
import {computed} from 'vue';
import {Popover, PopoverButton, PopoverPanel, Transition} from '@headlessui/vue';
import {SettingsIcon} from 'lucide-vue-next';
import VrcButton from './VrcButton.vue';
import UserAvatar from './UserAvatar.vue';
import type {AuthUser} from '../composables/useAuthFlow';

const props = defineProps<{
  user: AuthUser;
}>();

const emit = defineEmits<{
  (e: 'open-settings'): void;
  (e: 'logout'): void;
}>();

const accountAvatarUrl = computed(() => {
  const user = props.user;
  return (
    user.profilePicOverrideThumbnail ||
    user.currentAvatarThumbnailImageUrl ||
    user.userIcon ||
    user.imageUrl ||
    ''
  );
});

const handleLogout = () => {
  emit('logout');
};
</script>

<template>
  <Popover class="relative">
    <PopoverButton
        class="flex items-center justify-center p-0.5 rounded-full text-vrc-text transition hover:text-vrc-highlight"
        data-tauri-drag-region="false"
    >
      <UserAvatar
          :src="accountAvatarUrl"
          :name="user.displayName"
          size-class="size-8"
          border-class="border-0"
          fallback-class="font-semibold text-[10px]"
      />
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
          class="absolute bg-vrc-background-secondary border-2 border-vrc-highlight/30 mt-2 p-3 right-0 rounded-md shadow-[0_18px_30px_-24px_rgba(0,0,0,0.8)] text-vrc-text text-xs w-52"
          data-tauri-drag-region="false"
      >
        <div class="flex gap-2 items-center mb-3">
          <UserAvatar
              :src="accountAvatarUrl"
              :name="user.displayName"
              size-class="size-9"
              border-class="border border-vrc-highlight/30"
              fallback-class="font-semibold text-[11px]"
          />
          <div class="min-w-0">
            <p class="font-semibold truncate">{{ user.displayName }}</p>
            <p v-if="user.username" class="text-[10px] text-vrc-text/60">
              {{ user.username }}
            </p>
          </div>
        </div>
        <button
            type="button"
            class="border border-vrc-highlight/20 flex gap-2 items-center mb-2 px-3 py-2 rounded-md text-vrc-text text-xs transition w-full hover:border-vrc-highlight/60 hover:text-vrc-highlight"
            @click="emit('open-settings')"
        >
          <SettingsIcon :size="14" />
          設定
        </button>
        <VrcButton size="sm" variant="secondary" @click="handleLogout">ログアウト</VrcButton>
      </PopoverPanel>
    </Transition>
  </Popover>
</template>

