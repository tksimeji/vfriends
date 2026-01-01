<script setup lang="ts">
import {computed} from 'vue';
import {DynamicScroller, DynamicScrollerItem} from 'vue-virtual-scroller';
import FriendCard from './FriendCard.vue';
import {useFriendGrid} from '../../composables/useFriendGrid';
import type {VRChat} from '../../vrchat.ts';

const props = defineProps<{
  friends: VRChat.LimitedUserFriend[];
}>();

const emit = defineEmits<{
  (e: 'open-settings', friendId: string): void;
}>();

const friendsRef = computed(() => props.friends);
const {gridRows, gridStyle, minRowHeight, gridContainerRef} = useFriendGrid(friendsRef);

const openSettingsForFriend = (friendId: string) => {
  emit('open-settings', friendId);
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
            />
          </div>
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </div>
</template>

