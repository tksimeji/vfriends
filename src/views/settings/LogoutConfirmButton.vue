<script setup lang="ts">
import {Dialog, DialogPanel, DialogTitle, TransitionChild, TransitionRoot} from '@headlessui/vue';
import {ref} from 'vue';
import {useI18n} from 'vue-i18n';
import VrcButton from '../../components/VrcButton.vue';

const emit = defineEmits<{
  (e: 'confirm'): void;
}>();

const {t} = useI18n();

const isOpen = ref(false);

const open = () => {
  isOpen.value = true;
};

const close = () => {
  isOpen.value = false;
};

const handleConfirm = () => {
  emit('confirm');
  close();
};
</script>

<template>
  <div class="flex justify-center">
    <VrcButton
        size="sm"
        class="bg-red-500/10! border-red-500/40! text-red-200! hover:bg-red-500/20! hover:border-red-400! hover:text-red-100!"
        @click="open"
    >
      {{ t('settings.actions.logout') }}
    </VrcButton>
  </div>

  <TransitionRoot as="template" :show="isOpen">
    <Dialog class="fixed inset-0 select-none z-50" @close="close">
      <TransitionChild
          as="template"
          enter="duration-150 ease-out"
          enter-from="opacity-0"
          enter-to="opacity-100"
          leave="duration-100 ease-in"
          leave-from="opacity-100"
          leave-to="opacity-0"
      >
        <div class="bg-black/60 fixed inset-0"/>
      </TransitionChild>

      <div class="fixed inset-0 overflow-y-auto">
        <div class="flex items-center justify-center min-h-full p-4 text-center">
          <TransitionChild
              as="template"
              enter="duration-150 ease-out"
              enter-from="opacity-0 scale-95"
              enter-to="opacity-100 scale-100"
              leave="duration-100 ease-in"
              leave-from="opacity-100 scale-100"
              leave-to="opacity-0 scale-95"
          >
            <DialogPanel class="bg-vrc-background border border-vrc-highlight/20 max-w-sm p-5 rounded-xl text-left w-full">
              <DialogTitle class="font-semibold text-lg text-vrc-text">
                {{ t('settings.actions.logoutConfirmTitle') }}
              </DialogTitle>
              <p class="mt-2 text-sm text-vrc-text/70">
                {{ t('settings.actions.logoutConfirmDescription') }}
              </p>
              <div class="flex gap-3 justify-end mt-6">
                <VrcButton size="sm" @click="close">
                  {{ t('settings.actions.logoutCancel') }}
                </VrcButton>
                <VrcButton
                    size="sm"
                    class="bg-red-500/10! border-red-500/40! text-red-200! hover:bg-red-500/20! hover:border-red-400! hover:text-red-100!"
                    @click="handleConfirm"
                >
                  {{ t('settings.actions.logoutConfirm') }}
                </VrcButton>
              </div>
            </DialogPanel>
          </TransitionChild>
        </div>
      </div>
    </Dialog>
  </TransitionRoot>
</template>