import type {ComputedRef, Ref} from 'vue';
import {computed, onBeforeUnmount, ref, unref, watch} from 'vue';
import {VRChat} from '../vrchat.ts';

const MIN_CARD_WIDTH = 240;
const FALLBACK_GAP = 8;
const ROW_HEIGHT_BY_COLUMNS: Record<number, number> = {
  1: 360,
  2: 300,
  3: 270,
  4: 250,
  5: 230,
};

const entrySizeKey = (entry: VRChat.LimitedUserFriend) =>
  `${entry.id}:${entry.last_activity ?? ''}:${entry.last_login ?? ''}`;

const rowSizeKey = (entries: VRChat.LimitedUserFriend[]) =>
  entries.map(entrySizeKey).join('|');

const buildRows = (
  source: VRChat.LimitedUserFriend[],
  columns: number,
) => {
  const rows: { id: string; items: VRChat.LimitedUserFriend[]; sizeKey: string }[] = [];
  for (let index = 0; index < source.length; index += columns) {
    const rowItems = source.slice(index, index + columns);
    rows.push({
      id: `row-${index}`,
      items: rowItems,
      sizeKey: rowSizeKey(rowItems),
    });
  }
  return rows;
};

export const useFriendGrid = (
  items: Ref<VRChat.LimitedUserFriend[]> | ComputedRef<VRChat.LimitedUserFriend[]>,
) => {
  const gridContainerRef = ref<HTMLElement | null>(null);
  const columnCount = ref(1);
  const minRowHeight = computed(
    () => ROW_HEIGHT_BY_COLUMNS[columnCount.value] ?? 260,
  );
  const gridStyle = computed(() => ({
    gridTemplateColumns: `repeat(${columnCount.value}, minmax(0, 1fr))`,
  }));
  let cleanupObserver: (() => void) | null = null;

  const readGap = (target: HTMLElement) => {
    if (typeof window === 'undefined') return FALLBACK_GAP;
    const styles = window.getComputedStyle(target);
    const gapValue = styles.columnGap || styles.gap || '';
    const gap = Number.parseFloat(gapValue);
    return Number.isFinite(gap) ? gap : FALLBACK_GAP;
  };

  const computeColumns = (width: number, gap: number) => {
    if (!Number.isFinite(width) || width <= 0) return 1;
    const total = width + gap;
    const step = MIN_CARD_WIDTH + gap;
    return Math.max(1, Math.floor(total / step));
  };

  const updateColumns = (target: HTMLElement) => {
    const gap = readGap(target);
    const nextCount = computeColumns(target.clientWidth, gap);
    if (nextCount !== columnCount.value) {
      columnCount.value = nextCount;
    }
  };

  const gridRows = computed(
    (): { id: string; items: VRChat.LimitedUserFriend[]; sizeKey: string }[] =>
      buildRows(unref(items), Math.max(1, columnCount.value)),
  );

  watch(
    gridContainerRef,
    (target) => {
      if (cleanupObserver) {
        cleanupObserver();
        cleanupObserver = null;
      }
      if (!target || typeof window === 'undefined') return;
      updateColumns(target);
      const observer = new ResizeObserver(() => updateColumns(target));
      observer.observe(target);
      cleanupObserver = () => observer.disconnect();
    },
    {flush: 'post'},
  );

  onBeforeUnmount(() => {
    if (cleanupObserver) {
      cleanupObserver();
      cleanupObserver = null;
    }
  });

  return {
    gridContainerRef,
    gridRows,
    gridStyle,
    minRowHeight,
  };
};
