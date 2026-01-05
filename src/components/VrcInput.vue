<script setup lang="ts">
import {EyeIcon, EyeOffIcon} from 'lucide-vue-next';
import {computed, InputTypeHTMLAttribute, ref, useAttrs} from 'vue';
import {useI18n} from 'vue-i18n';

defineOptions({inheritAttrs: false});

const props = withDefaults(defineProps<{
  type?: InputTypeHTMLAttribute;
  label?: string;
  disabled?: boolean;
}>(), {
  type: 'text',
  label: '',
  disabled: false,
});

const attrs = useAttrs();
const {t} = useI18n();

const showPassword = ref(false);
const inputRef = ref<HTMLInputElement | null>(null);

const isDisabled = computed(() => props.disabled || Boolean(attrs.disabled));
const isPassword = computed(() => props.type === 'password');
const inputType = computed(() => isPassword.value ? (showPassword.value ? 'text' : 'password') : props.type);

const togglePassword = () => {
  showPassword.value = !showPassword.value;
};

const focus = () => {
  inputRef.value?.focus();
};

defineExpose({
  focus,
});
</script>

<template>
  <div class="select-none space-y-1" :class="isDisabled ? 'cursor-not-allowed' : ''">
    <label v-if="props.label" class="block font-semibold text-md text-vrc-text">
      {{ props.label }}
    </label>
    <div
        class="border-b-2 flex gap-2 items-center px-3 py-2 rounded-md text-left text-sm transition w-full"
        :class="isDisabled ? 'bg-vrc-button/50 border-b-vrc-highlight/10 text-vrc-text/40' : 'bg-vrc-background/90 border-b-vrc-highlight/40 text-vrc-text focus-within:border-b-vrc-highlight/70 hover:bg-vrc-background/60 hover:border-b-vrc-highlight/60'"
    >
      <div v-if="$slots.default" class="text-vrc-subtext">
        <slot/>
      </div>

      <input
          v-bind="attrs"
          ref="inputRef"
          class="bg-transparent grow min-w-0 outline-none text-sm placeholder:text-vrc-subtext"
          :class="isDisabled ? 'cursor-not-allowed' : ''"
          :disabled="isDisabled"
          :type="inputType"
      />

      <button
          v-if="isPassword"
          type="button"
          class="cursor-pointer px-2 text-vrc-subtext transition hover:text-vrc-highlight"
          @click="togglePassword"
          :aria-label="showPassword ? t('input.hidePassword') : t('input.showPassword')"
      >
        <EyeOffIcon v-if="showPassword"/>
        <EyeIcon v-else/>
      </button>
    </div>
  </div>
</template>
