<script setup lang="ts">
import {computed, ref} from 'vue';
import {DynamicScroller, DynamicScrollerItem} from 'vue-virtual-scroller';
import {useFriendGrid} from '../../composables/useFriendGrid';
import type {VRChat} from '../../vrchat.ts';
import FriendCard from './FriendCard.vue';

const props = defineProps<{
  friends: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'open-settings', friendId: string): void;
  (e: 'hover-color', rgb: [number, number, number] | null): void;
}>();

const friendsRef = computed(() => props.friends);
const {gridRows, gridStyle, minRowHeight, gridContainerRef} = useFriendGrid(friendsRef);
const hoveredId = ref<string | null>(null);

const openSettingsForFriend = (friendId: string) => {
  emit('open-settings', friendId);
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
</script>

<template>
  <div ref="gridContainerRef" class="flex-1 min-h-0">
    <DynamicScroller
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
          <div class="gap-2 grid h-full pb-2" :style="gridStyle">
            <FriendCard
                v-for="entry in item.items"
                :key="entry.id"
                :friend="entry"
                @open-settings="openSettingsForFriend(entry.id)"
                @hover="handleHover"
            />
          </div>
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </div>
</template>