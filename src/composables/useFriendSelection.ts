import {computed, ref} from 'vue';
import type {VRChat} from '../vrchat.ts';

type RangeSelectionMode = 'replace' | 'add';

type SetSelectionOptions = {
  silent?: boolean;
};

type FriendSelectionOptions = {
  getRangeList: () => VRChat.LimitedUserFriend[];
  onChange?: () => void;
};

const indexFriendsById = (list: VRChat.LimitedUserFriend[]) =>
  new Map(list.map((friend, index) => [friend.id, index]));

const buildRangeSelection = (
  list: VRChat.LimitedUserFriend[],
  startIndex: number,
  endIndex: number,
) => {
  const next = new Set<string>();
  const start = Math.max(0, Math.min(startIndex, endIndex));
  const end = Math.min(list.length - 1, Math.max(startIndex, endIndex));
  for (let index = start; index <= end; index += 1) {
    next.add(list[index].id);
  }
  return next;
};

const resolveAnchorIndex = (targetIndex: number, minIndex: number, maxIndex: number) =>
  targetIndex < minIndex
    ? minIndex
    : targetIndex > maxIndex
    ? maxIndex
    : (() => {
        const distanceToTop = targetIndex - minIndex;
        const distanceToBottom = maxIndex - targetIndex;
        return distanceToTop <= distanceToBottom ? minIndex : maxIndex;
      })();

export const useFriendSelection = (options: FriendSelectionOptions) => {
  const selectedIds = ref<Set<string>>(new Set());
  const selectionAnchorId = ref<string | null>(null);
  const hasSelection = computed(() => selectedIds.value.size > 0);
  let onChange: (() => void) | null = options.onChange ?? null;

  const setOnChange = (handler: (() => void) | null) => {
    onChange = handler;
  };

  const notifyChange = (nextOptions?: SetSelectionOptions) => {
    if (nextOptions?.silent) return;
    onChange?.();
  };

  const setSelection = (
    next: Set<string>,
    anchorId?: string | null,
    nextOptions?: SetSelectionOptions,
  ) => {
    selectedIds.value = next;
    if (next.size === 0) {
      selectionAnchorId.value = null;
      notifyChange(nextOptions);
      return;
    }
    if (anchorId && next.has(anchorId)) {
      selectionAnchorId.value = anchorId;
      notifyChange(nextOptions);
      return;
    }
    selectionAnchorId.value = next.values().next().value ?? null;
    notifyChange(nextOptions);
  };

  const clearSelection = (nextOptions?: SetSelectionOptions) => {
    if (selectedIds.value.size === 0) return;
    setSelection(new Set(), null, nextOptions);
  };

  const selectSingle = (friendId: string, nextOptions?: SetSelectionOptions) => {
    setSelection(new Set([friendId]), friendId, nextOptions);
  };

  const toggleSelection = (friendId: string, nextOptions?: SetSelectionOptions) => {
    const next = new Set(selectedIds.value);
    const wasSelected = next.has(friendId);
    if (wasSelected) {
      next.delete(friendId);
    } else {
      next.add(friendId);
    }
    if (!wasSelected) {
      setSelection(next, friendId, nextOptions);
      return;
    }
    setSelection(next, selectionAnchorId.value, nextOptions);
  };

  const applyRangeSelection = (friendId: string, mode: RangeSelectionMode) => {
    const list = options.getRangeList();
    if (list.length === 0) return;
    const indexMap = indexFriendsById(list);
    const targetIndex = indexMap.get(friendId);
    if (targetIndex === undefined) return;
    const selectedIndices = [...selectedIds.value]
      .map((id) => indexMap.get(id))
      .filter((index): index is number => typeof index === 'number');
    if (selectedIndices.length === 0) {
      selectSingle(friendId);
      return;
    }
    const minIndex = Math.min(...selectedIndices);
    const maxIndex = Math.max(...selectedIndices);
    const anchorIndex = resolveAnchorIndex(targetIndex, minIndex, maxIndex);
    const range = buildRangeSelection(list, anchorIndex, targetIndex);
    if (mode === 'add') {
      const merged = new Set(selectedIds.value);
      range.forEach((id) => merged.add(id));
      setSelection(merged, list[anchorIndex]?.id ?? friendId);
      return;
    }
    setSelection(range, list[anchorIndex]?.id ?? friendId);
  };

  const selectAll = (list: VRChat.LimitedUserFriend[]) => {
    if (list.length === 0) {
      clearSelection();
      return;
    }
    setSelection(new Set(list.map((friend) => friend.id)), list[0]?.id ?? null);
  };

  const pruneSelection = (list: VRChat.LimitedUserFriend[]) => {
    const allowed = new Set(list.map((friend) => friend.id));
    const filtered = new Set(
      [...selectedIds.value].filter((id) => allowed.has(id)),
    );
    setSelection(filtered, selectionAnchorId.value);
  };

  return {
    selectedIds,
    selectionAnchorId,
    hasSelection,
    setSelection,
    clearSelection,
    selectSingle,
    toggleSelection,
    applyRangeSelection,
    selectAll,
    pruneSelection,
    setOnChange,
  };
};
