<script setup lang="ts">
import {computed, onBeforeUnmount, onMounted} from 'vue';
import {DynamicScroller, DynamicScrollerItem} from 'vue-virtual-scroller';
import {useFriends} from '../../composables/useFriends';
import {useFriendGrid} from '../../composables/useFriendGrid';
import FriendCard from './FriendCard.vue';

const {
  sortedItems,
  isLoading,
  errorMessage,
  hasFriends,
  refresh,
  startAutoRefresh,
  stopAutoRefresh,
} = useFriends();

const {
  gridRows,
  gridStyle,
  minRowHeight,
  gridContainerRef,
} = useFriendGrid(sortedItems);

const statusMessage = computed(() => {
  if (errorMessage.value) {
    return {
      text: errorMessage.value,
      tone: 'error',
    };
  }
  if (isLoading.value) {
    return {
      text: 'フレンド一覧を読み込み中...',
      tone: 'muted',
    };
  }
  if (!hasFriends.value) {
    return {
      text: 'フレンドが見つかりません。',
      tone: 'muted',
    };
  }
  return null;
});

onMounted(() => {
  void refresh();
  startAutoRefresh();
});

onBeforeUnmount(() => {
  stopAutoRefresh();
});
</script>

<template>
  <div
    ref="gridContainerRef"
    class="relative mx-auto flex w-full max-w-6xl flex-1 flex-col min-h-0"
  >
    <p
        v-if="statusMessage"
        class="text-sm"
        :class="statusMessage.tone === 'error' ? 'text-red-300' : 'text-vrc-text/70'"
    >
      {{ statusMessage.text }}
    </p>

    <DynamicScroller
        v-else
        class="flex-1 h-full min-h-0 overflow-auto"
        :items="gridRows"
        :min-item-size="minRowHeight"
    >
      <template #default="{ item, active }">
        <DynamicScrollerItem
            :item="item"
            :active="active"
            :size-dependencies="[item.sizeKey]"
        >
          <div class="grid h-full gap-2 pb-2" :style="gridStyle">
            <FriendCard
                v-for="entry in item.items"
                :key="entry.id"
                :friend="entry"
            />
          </div>
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </div>
</template>
