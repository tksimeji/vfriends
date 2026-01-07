<script setup lang="ts">
import {computed, ref} from 'vue';
import {DynamicScroller, DynamicScrollerItem} from 'vue-virtual-scroller';
import {useFriendGrid} from '../../composables/useFriendGrid';
import type {VRChat} from '../../vrchat.ts';
import FriendCard from './FriendCard.vue';

type FriendsListHandle = {
  getSelectoContainer: () => HTMLElement | null;
  getScrollContainer: () => HTMLElement | null;
};

const props = defineProps<{
  friends: VRChat.LimitedUserFriend[];
  selectedIds: Set<string>;
  settingsVersion: number;
}>();

const emit = defineEmits<{
  (e: 'friend-click', payload: { friend: VRChat.LimitedUserFriend; event: MouseEvent }): void;
  (e: 'hover-color', rgb: [number, number, number] | null): void;
}>();

const friendsRef = computed(() => props.friends);
const {gridRows, gridStyle, minRowHeight, gridContainerRef} = useFriendGrid(friendsRef);
const scrollerRef = ref<{ $el: HTMLElement } | null>(null);
const hoveredId = ref<string | null>(null);

const handleFriendClick = (payload: { friend: VRChat.LimitedUserFriend; event: MouseEvent }) => {
  emit('friend-click', payload);
};

const handleHover = (payload: {
  id: string;
  rgb: [number, number, number] | null;
  active: boolean;
}) => {
  if (payload.active) {
    hoveredId.value = payload.id;
    emit('hover-color', payload.rgb);
    return;
  }
  if (hoveredId.value === payload.id) {
    hoveredId.value = null;
    emit('hover-color', null);
  }
};

const isSelected = (friendId: string) => props.selectedIds.has(friendId);

defineExpose<FriendsListHandle>({
  getSelectoContainer: () => gridContainerRef.value,
  getScrollContainer: () => scrollerRef.value?.$el ?? null,
});
</script>

<template>
  <div class="flex-1 min-h-0 relative">
    <div
        ref="gridContainerRef"
        aria-hidden="true"
        class="h-0 max-w-6xl mx-auto px-4 w-full"
    ></div>
    <DynamicScroller
        ref="scrollerRef"
        class="h-full overflow-y-auto"
        :items="gridRows"
        :min-item-size="minRowHeight"
    >
      <template #default="{ item, active }">
        <DynamicScrollerItem
            :item="item"
            :active="active"
            :size-dependencies="[item.sizeKey]"
        >
          <div class="max-w-6xl mx-auto px-4">
            <div class="gap-2 grid h-full pb-2" :style="gridStyle">
              <FriendCard
                  v-for="entry in item.items"
                  :key="entry.id"
                  :friend="entry"
                  :selected="isSelected(entry.id)"
                  :settings-version="props.settingsVersion"
                  @card-click="handleFriendClick"
                  @hover="handleHover"
              />
            </div>
          </div>
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </div>
</template>