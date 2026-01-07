import {nextTick, onBeforeUnmount, onMounted, ref, watch, type Ref} from 'vue';
import Selecto from 'selecto';
import type {VRChat} from '../vrchat.ts';

type FriendsListHandle = {
  getSelectoContainer: () => HTMLElement | null;
  getScrollContainer: () => HTMLElement | null;
};

type SelectionSetOptions = {
  silent?: boolean;
};

type FriendSelectionDragOptions = {
  viewportRef: Ref<HTMLElement | null>;
  listRef: Ref<FriendsListHandle | null>;
  showList: Ref<boolean>;
  isModalOpen?: Ref<boolean>;
  getFilteredFriends: () => VRChat.LimitedUserFriend[];
  selectedIds: Ref<Set<string>>;
  selectionAnchorId: Ref<string | null>;
  setSelection: (next: Set<string>, anchorId?: string | null, options?: SelectionSetOptions) => void;
  clearSelection: () => void;
  selectAll: (list: VRChat.LimitedUserFriend[]) => void;
  setOnSelectionChange: (handler: (() => void) | null) => void;
};

const readInputPoint = (inputEvent: Event | undefined | null) => {
  if (!inputEvent) return null;
  if (typeof TouchEvent !== 'undefined' && inputEvent instanceof TouchEvent) {
    const touch = inputEvent.touches[0] ?? inputEvent.changedTouches[0];
    if (!touch) return null;
    return {x: touch.clientX, y: touch.clientY};
  }
  if (inputEvent instanceof MouseEvent) {
    return {x: inputEvent.clientX, y: inputEvent.clientY};
  }
  const fallback = inputEvent as { clientX?: number; clientY?: number };
  if (typeof fallback.clientX === 'number' && typeof fallback.clientY === 'number') {
    return {x: fallback.clientX, y: fallback.clientY};
  }
  return null;
};

const getContentRect = (scrollContainer: HTMLElement, rect: DOMRect) => {
  const containerRect = scrollContainer.getBoundingClientRect();
  return {
    left: rect.left - containerRect.left + scrollContainer.scrollLeft,
    right: rect.right - containerRect.left + scrollContainer.scrollLeft,
    top: rect.top - containerRect.top + scrollContainer.scrollTop,
    bottom: rect.bottom - containerRect.top + scrollContainer.scrollTop,
  };
};

const rectsIntersect = (
  rect: { left: number; right: number; top: number; bottom: number },
  target: { left: number; right: number; top: number; bottom: number },
) => rect.left <= target.right
  && rect.right >= target.left
  && rect.top <= target.bottom
  && rect.bottom >= target.top;

export const useFriendSelectionDrag = (options: FriendSelectionDragOptions) => {
  const isSelecting = ref(false);
  const selectoRef = ref<Selecto | null>(null);
  const dragState = ref<{
    start: { x: number; y: number };
    current: { x: number; y: number };
    scrollStart: { top: number; left: number };
    additive: boolean;
    moved: boolean;
  } | null>(null);
  let cleanupScrollListener: (() => void) | null = null;
  let autoScrollRafId: number | null = null;
  let autoScrollContainer: HTMLElement | null = null;

  const syncSelectoSelection = () => {
    const selecto = selectoRef.value;
    const container = options.listRef.value?.getScrollContainer() ?? options.viewportRef.value;
    if (!selecto || !container) return;
    if (options.selectedIds.value.size === 0) {
      selecto.setSelectedTargets([]);
      return;
    }
    const elements = Array.from(container.querySelectorAll<HTMLElement>('.friend-card-selectable'));
    const selectedElements = elements.filter((element) => {
      const id = element.dataset.friendId;
      return id ? options.selectedIds.value.has(id) : false;
    });
    selecto.setSelectedTargets(selectedElements);
  };

  options.setOnSelectionChange(syncSelectoSelection);

  const getSelectionRectInContent = (scrollContainer: HTMLElement) => {
    if (!dragState.value) return null;
    const containerRect = scrollContainer.getBoundingClientRect();
    const {start, current, scrollStart} = dragState.value;
    const startX = start.x - containerRect.left + scrollStart.left;
    const startY = start.y - containerRect.top + scrollStart.top;
    const currentX = current.x - containerRect.left + scrollContainer.scrollLeft;
    const currentY = current.y - containerRect.top + scrollContainer.scrollTop;
    return {
      left: Math.min(startX, currentX),
      right: Math.max(startX, currentX),
      top: Math.min(startY, currentY),
      bottom: Math.max(startY, currentY),
    };
  };

  const updateSelectionFromDrag = (scrollContainer: HTMLElement) => {
    const selectionRect = getSelectionRectInContent(scrollContainer);
    if (!selectionRect) return;
    if (dragState.value) {
      const dx = Math.abs(dragState.value.current.x - dragState.value.start.x);
      const dy = Math.abs(dragState.value.current.y - dragState.value.start.y);
      const scrollDx = Math.abs(scrollContainer.scrollLeft - dragState.value.scrollStart.left);
      const scrollDy = Math.abs(scrollContainer.scrollTop - dragState.value.scrollStart.top);
      const moved = dx > 4 || dy > 4 || scrollDx > 1 || scrollDy > 1;
      dragState.value.moved = moved;
      isSelecting.value = moved;
    }
    const selectableElements = Array.from(
      scrollContainer.querySelectorAll<HTMLElement>('.friend-card-selectable'),
    );
    const hits = new Set<string>();
    selectableElements.forEach((element) => {
      const id = element.dataset.friendId;
      if (!id) return;
      const rect = getContentRect(scrollContainer, element.getBoundingClientRect());
      if (rectsIntersect(selectionRect, rect)) {
        hits.add(id);
      }
    });
    if (dragState.value?.additive) {
      const merged = new Set(options.selectedIds.value);
      hits.forEach((id) => merged.add(id));
      options.setSelection(merged, options.selectionAnchorId.value, {silent: true});
      return;
    }
    const anchor = hits.values().next().value ?? options.selectionAnchorId.value;
    options.setSelection(hits, anchor, {silent: true});
  };

  const updateAutoScroll = () => {
    autoScrollRafId = null;
    if (!dragState.value || !autoScrollContainer) return;
    const rect = autoScrollContainer.getBoundingClientRect();
    const point = dragState.value.current;
    const threshold = 48;
    const maxSpeed = 16;
    let deltaY = 0;
    if (point.y < rect.top + threshold) {
      const ratio = (threshold - (point.y - rect.top)) / threshold;
      deltaY = -Math.ceil(maxSpeed * ratio);
    } else if (point.y > rect.bottom - threshold) {
      const ratio = (threshold - (rect.bottom - point.y)) / threshold;
      deltaY = Math.ceil(maxSpeed * ratio);
    }
    if (deltaY !== 0) {
      autoScrollContainer.scrollTop += deltaY;
      if (dragState.value) {
        dragState.value.moved = true;
        isSelecting.value = true;
      }
      updateSelectionFromDrag(autoScrollContainer);
    }
    autoScrollRafId = requestAnimationFrame(updateAutoScroll);
  };

  const startAutoScroll = () => {
    if (autoScrollRafId) return;
    autoScrollRafId = requestAnimationFrame(updateAutoScroll);
  };

  const stopAutoScroll = () => {
    if (autoScrollRafId) {
      cancelAnimationFrame(autoScrollRafId);
      autoScrollRafId = null;
    }
  };

  const setupSelecto = async () => {
    await nextTick();
    const listContainer = options.listRef.value?.getSelectoContainer();
    const container = options.viewportRef.value;
    const scrollContainer = options.listRef.value?.getScrollContainer();
    if (!listContainer || !container || !scrollContainer) return;
    const dragContainer = container;
    autoScrollContainer = scrollContainer;
    if (selectoRef.value) {
      selectoRef.value.destroy();
      selectoRef.value = null;
    }
    if (cleanupScrollListener) {
      cleanupScrollListener();
      cleanupScrollListener = null;
    }
    const selecto = new Selecto({
      container,
      rootContainer: null,
      dragContainer,
      selectableTargets: ['.friend-card-selectable'],
      className: 'selecto-selection',
      selectByClick: false,
      selectFromInside: true,
      continueSelect: false,
      toggleContinueSelect: 'ctrl',
      keyContainer: window,
      preventDragFromInside: true,
      dragCondition: (event) => {
        const target = event.inputEvent?.target as HTMLElement | undefined;
        if (!target) return true;
        if (target.closest('[data-selection-actions]')) return false;
        if (target.closest('[data-bulk-toast]')) return false;
        return true;
      },
      scrollOptions: {
        container: scrollContainer,
      },
      hitRate: 0,
    });
    selecto.options.selectableTargets = ['.friend-card-selectable'];
    selecto.on('dragStart', (event) => {
      isSelecting.value = false;
      const inputEvent = event.inputEvent as MouseEvent | undefined;
      const target = inputEvent?.target as HTMLElement | undefined;
      const isSelectable = target?.closest?.('.friend-card-selectable');
      if (!isSelectable && !inputEvent?.ctrlKey && !inputEvent?.metaKey) {
        options.clearSelection();
      }
      const point = readInputPoint(event.inputEvent);
      if (point) {
        dragState.value = {
          start: point,
          current: point,
          scrollStart: {
            top: scrollContainer.scrollTop,
            left: scrollContainer.scrollLeft,
          },
          additive: Boolean(inputEvent && (inputEvent.ctrlKey || inputEvent.metaKey)),
          moved: false,
        };
      } else {
        dragState.value = null;
      }
      startAutoScroll();
    });
    selecto.on('drag', (event) => {
      const point = readInputPoint(event.inputEvent);
      if (point && dragState.value) {
        dragState.value.current = point;
      }
      if (dragState.value?.moved) {
        updateSelectionFromDrag(scrollContainer);
      }
    });
    selecto.on('select', (event) => {
      const point = readInputPoint(event.inputEvent);
      if (point && dragState.value) {
        dragState.value.current = point;
      }
      updateSelectionFromDrag(scrollContainer);
    });
    selecto.on('selectEnd', () => {
      dragState.value = null;
      requestAnimationFrame(() => {
        isSelecting.value = false;
        syncSelectoSelection();
        stopAutoScroll();
      });
    });
    selectoRef.value = selecto;
    const handleScroll = () => {
      if (!selectoRef.value || !isSelecting.value || !dragState.value) return;
      updateSelectionFromDrag(scrollContainer);
    };
    scrollContainer.addEventListener('scroll', handleScroll, {passive: true});
    cleanupScrollListener = () => {
      scrollContainer.removeEventListener('scroll', handleScroll);
    };
    syncSelectoSelection();
  };

  const handleViewportPointerDown = (event: PointerEvent) => {
    const target = event.target as HTMLElement | null;
    if (!target) return;
    if (target.closest('.friend-card-selectable')) return;
    if (target.closest('.selecto-selection')) return;
    if (target.closest('[data-selection-actions]')) return;
    if (target.closest('[data-bulk-toast]')) return;
    if (event.ctrlKey || event.metaKey) return;
    options.clearSelection();
  };

  const isEditableTarget = (target: EventTarget | null) => {
    if (!(target instanceof HTMLElement)) return false;
    if (target.isContentEditable) return true;
    const tagName = target.tagName.toLowerCase();
    return tagName === 'input' || tagName === 'textarea' || tagName === 'select';
  };

  const handleKeydown = (event: KeyboardEvent) => {
    if (isEditableTarget(event.target)) return;
    if (options.isModalOpen?.value && event.key.toLowerCase() === 'a' && event.ctrlKey) {
      return;
    }
    if (event.key === 'Escape') {
      options.clearSelection();
      return;
    }
    if (!event.ctrlKey || event.metaKey) return;
    const lowerKey = event.key.toLowerCase();
    if (lowerKey === 'a' && event.shiftKey) {
      event.preventDefault();
      options.clearSelection();
      return;
    }
    if (lowerKey === 'a') {
      event.preventDefault();
      options.selectAll(options.getFilteredFriends());
    }
  };

  onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
  });

  onBeforeUnmount(() => {
    selectoRef.value?.destroy();
    selectoRef.value = null;
    if (cleanupScrollListener) {
      cleanupScrollListener();
      cleanupScrollListener = null;
    }
    if (autoScrollRafId) {
      cancelAnimationFrame(autoScrollRafId);
      autoScrollRafId = null;
    }
    autoScrollContainer = null;
    window.removeEventListener('keydown', handleKeydown);
  });

  watch(
    () => options.showList.value,
    (next) => {
      if (!next) return;
      setupSelecto();
    },
  );

  watch(
    () => options.listRef.value,
    () => {
      if (!options.showList.value) return;
      setupSelecto();
    },
    {flush: 'post'},
  );

  return {
    isSelecting,
    handleViewportPointerDown,
  };
};
