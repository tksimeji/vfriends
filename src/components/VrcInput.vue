<script setup lang="ts">
import {EyeIcon, EyeOffIcon} from 'lucide-vue-next';
import {computed, InputTypeHTMLAttribute, ref, useAttrs} from 'vue';
import {useI18n} from 'vue-i18n';

defineOptions({inheritAttrs: false});

const props = withDefaults(defineProps<{
  type?: InputTypeHTMLAttribute;
  label?: string;
  helper?: string;
  disabled?: boolean;
}>(), {
  type: 'text',
  label: '',
  helper: '',
  disabled: false,
});

const attrs = useAttrs();
const {t} = useI18n();

const showPassword = ref(false);
const isPassword = computed(() => props.type === 'password');
const inputType = computed(() =>
  isPassword.value ? (showPassword.value ? 'text' : 'password') : props.type,
);

const isDisabled = computed(() => props.disabled || Boolean(attrs.disabled));

const containerClasses = computed(() => {
  const base =
    'border-2 flex gap-2 items-center px-3 py-2 rounded-md text-left text-sm transition w-full focus-within:ring-2 focus-within:ring-vrc-highlight/40';
  const state = isDisabled.value
    ? 'bg-vrc-button/50 border-vrc-highlight/10 cursor-not-allowed text-vrc-text/40'
    : 'bg-vrc-button/80 border-vrc-highlight/20 text-vrc-text focus-within:border-vrc-highlight/70 hover:border-vrc-highlight/60';
  return `${base} ${state}`;
});

const inputAttrs = computed(() => {
  const {class: _class, style: _style, ...rest} = attrs;
  return rest;
});

const togglePassword = () => {
  showPassword.value = !showPassword.value;
};
</script>

<template>
  <div class="space-y-1">
    <label v-if="props.label" class="block font-semibold text-md text-vrc-text">
      {{ props.label }}
    </label>
    <div :class="[containerClasses, attrs.class]" :style="attrs.style">
      <div v-if="$slots.default" class="text-vrc-subtext">
        <slot/>
      </div>

      <input
          class="bg-transparent grow min-w-0 outline-none text-sm placeholder:text-vrc-subtext/80"
          v-bind="inputAttrs"
          :disabled="isDisabled"
          :type="inputType"
      />

      <button
          v-if="isPassword"
          type="button"
          class="cursor-pointer px-2 text-vrc-subtext transition hover:text-vrc-highlight"
          @click="togglePassword"
          :aria-label="showPassword ? t('common.hidePassword') : t('common.showPassword')"
      >
        <EyeOffIcon v-if="showPassword"/>
        <EyeIcon v-else/>
      </button>
    </div>
    <p v-if="props.helper" class="text-[10px] text-vrc-text/60">{{ props.helper }}</p>
  </div>
</template>

