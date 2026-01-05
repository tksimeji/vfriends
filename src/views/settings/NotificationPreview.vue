<script setup lang="ts">
import {EllipsisIcon, PlayIcon, XIcon} from 'lucide-vue-next';
import {computed, onMounted} from 'vue';
import VrcAvatar from '../../components/VrcAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import {useAppSettings} from '../../composables/useAppSettings';
import {t} from '../../i18n.ts';
import {previewNotificationSound} from '../../invokes.ts';
import {FriendSettings} from '../../types.ts';
import {VRChat} from '../../vrchat.ts';

const props = defineProps<{
  user: VRChat.LimitedUserFriend;
  settings: FriendSettings | null;
}>();

const {appSettings, refresh} = useAppSettings();
onMounted(() => {
  void refresh();
});

const replacePlaceholder = (input: string, value: string) =>
    input.split('%s').join(value);

const message = computed(() => {
  const messageOverride = props.settings?.messageOverride;
  if (props.settings?.useOverride && messageOverride != null) {
    return replacePlaceholder(messageOverride, props.user.displayName);
  }

  const defaultMessage = appSettings.value?.defaultMessage ?? '%s is now online!';
  return replacePlaceholder(defaultMessage, props.user.displayName);
});

const handlePlayNotificationSound = () => {
  const friendSettings = props.settings;
  if (friendSettings?.useOverride && friendSettings.soundOverride) {
    previewNotificationSound(friendSettings.soundOverride);
  } else {
    previewNotificationSound(appSettings.value.defaultSound ?? null);
  }
};
</script>

<template>
  <div class="border-2 border-vrc-highlight h-86 max-w-lg overflow-hidden relative rounded-lg select-none w-full">
    <img
        class="absolute h-full inset-0 object-cover w-full"
        src="../../assets/WindowsBloom.jpg"
        alt="Windows Bloom"
    />

    <div class="absolute flex inset-0 items-center justify-center">
      <div
          class="bg-gray-800/80 backdrop-blur-md border-2 border-gray-600/80 flex flex-col gap-2 px-6 py-3 rounded-lg shadow-xl w-3/4">
        <div class="flex items-center justify-between">
          <p class="text-[10px] text-white">VFriends</p>
          <div class="flex gap-4 items-center">
            <EllipsisIcon :size="16"/>
            <XIcon :size="16"/>
          </div>
        </div>

        <div class="flex gap-4 items-center">
          <VrcAvatar :user="props.user" :size="64"/>
          <div>
            <p class="text-lg text-white">{{ props.user.displayName }}</p>
            <p class="text-md">{{ message }}</p>
          </div>
        </div>
      </div>
    </div>

    <VrcButton class="absolute bottom-3 right-3" @click="handlePlayNotificationSound">
      {{ t('settings.friend.previewSound') }}
      <PlayIcon :size="16"/>
    </VrcButton>
  </div>
</template>

<style scoped>

</style>
