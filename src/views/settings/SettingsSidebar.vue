<script setup lang="ts">
import {SearchIcon, VolumeOffIcon} from 'lucide-vue-next';
import {computed, nextTick, onBeforeUnmount, ref, watch} from 'vue';
import {RecycleScroller} from 'vue-virtual-scroller';
import StatusBadge from '../../components/StatusBadge.vue';
import UserAvatar from '../../components/UserAvatar.vue';
import VrcButton from '../../components/VrcButton.vue';
import VrcInput from '../../components/VrcInput.vue';
import {VRChat} from '../../vrchat.ts';
import {useI18n} from 'vue-i18n';

const props = defineProps<{
  currentUser: VRChat.CurrentUser | null;
  friends: VRChat.LimitedUserFriend[];
  selectedId: string;
  scrollTargetId?: string | null;
  isFriendEnabled: (friendId: string) => boolean;
}>();

const emit = defineEmits<{
  (e: 'select', id: string): void;
  (e: 'scrolled'): void;
}>();

type RecycleScrollerHandle = {
  scrollToItem: (index: number) => void;
  scrollToPosition: (position: number) => void;
};

const scrollerRef = ref<RecycleScrollerHandle | null>(null);
const isAlive = ref(true);
const searchQuery = ref('');
const {t} = useI18n();
const filteredFriends = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  if (!query) return props.friends;
  return props.friends.filter((friend) =>
      friend.displayName.toLowerCase().includes(query),
  );
});

const scrollToSelected = async () => {
  const scroller = scrollerRef.value;
  if (!scroller) return;
  await nextTick();
  const targetId = props.scrollTargetId;
  if (!targetId || targetId === 'global') {
    scroller.scrollToPosition(0);
    return;
  }
  const index = filteredFriends.value.findIndex(
      (friend) => friend.id === targetId,
  );
  if (index < 0) return;
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      scroller.scrollToItem(index);
    });
  });
};

const selectGlobal = () => {
  emit('select', 'global');
};

const selectFriend = (friendId: string) => {
  emit('select', friendId);
};

watch(
    () => [props.scrollTargetId, filteredFriends.value.length],
    () => {
      if (!props.scrollTargetId) return;
      if (!scrollerRef.value) return;
      void scrollToSelected().then(() => {
        if (!isAlive.value) return;
        emit('scrolled');
      });
    },
    {flush: 'post'},
);

watch(
    scrollerRef,
    (next) => {
      if (!next || !props.scrollTargetId) return;
      void scrollToSelected().then(() => {
        if (!isAlive.value) return;
        emit('scrolled');
      });
    },
    {flush: 'post'},
);

watch(
    () => [props.selectedId, props.scrollTargetId],
    ([nextSelected, nextScroll]) => {
      if (nextScroll || nextSelected === 'global') {
        searchQuery.value = '';
      }
    },
);

onBeforeUnmount(() => {
  isAlive.value = false;
});
</script>

<template>
  <aside class="bg-vrc-background-tertiary border-r border-vrc-highlight/20 flex flex-col w-60">
    <div class="flex flex-col gap-2 p-2">
      <VrcButton
          :class="selectedId === 'global' ? 'bg-vrc-highlight/15 border-vrc-highlight' : ''"
          @click="selectGlobal"
      >
        <UserAvatar
            v-if="currentUser"
            :user="currentUser"
            :size="40"
            fallback-class="font-semibold text-[10px]"
        />
        <span class="flex-1 font-semibold min-w-0 truncate">{{ t('common.appSettings') }}</span>
      </VrcButton>
      <VrcInput
          :placeholder="t('friends.searchPlaceholder')"
          :value="searchQuery"
          @input="searchQuery = ($event.target as HTMLInputElement).value"
      >
        <SearchIcon slot="" :size="16"/>
      </VrcInput>
    </div>

    <RecycleScroller
        ref="scrollerRef"
        class="flex-1 overflow-y-auto"
        key-field="id"
        :items="filteredFriends"
        :item-size="56"
    >
      <template #default="{ item }">
        <div class="mx-2 py-1 w-full">
          <button
              type="button"
              class="bg-linear-to-l flex from-vrc-background/20  gap-2 items-center px-4 py-3 rounded-md rounded-bl-2xl select-none to-vrc-background transition w-[calc(100%-1rem)]"
              :class="selectedId === item.id ? 'outline-2 outline-vrc-highlight to-vrc-highlight/15' : 'outline-vrc-highlight/40 hover:outline-2'"
              @click="selectFriend(item.id)"
          >
            <UserAvatar
                :user="item"
                :size="24"
                fallback-class="font-semibold text-[10px]"
            />
            <span class="font-semibold text-sm text-vrc-text truncate">{{ item.displayName }}</span>

            <div class="flex gap-2 ml-auto">
              <VolumeOffIcon
                  v-if="!isFriendEnabled(item.id)"
                  class="text-red-600"
                  :size="14"
              />
              <StatusBadge
                  :friend="item"
                  :show-label="false"
                  :size="12"
              />
            </div>
          </button>
        </div>
      </template>
    </RecycleScroller>
  </aside>
</template>
