<script setup lang="ts">
import {EyeIcon, EyeOffIcon} from 'lucide-vue-next';
import {computed, InputTypeHTMLAttribute, ref} from 'vue';

defineOptions({inheritAttrs: false});

const props = defineProps<{ type?: InputTypeHTMLAttribute }>();

const showPassword = ref(false);

const isPassword = computed(() => (props.type ?? 'text') === 'password');
const inputType = computed(() => isPassword.value ? (showPassword.value ? 'text' : 'password') : (props.type ?? 'text'));

const togglePassword = () => {
  showPassword.value = !showPassword.value;
};
</script>

<template>
  <div
      class="bg-vrc-button/80 border-2 border-vrc-highlight/20 flex gap-2 group items-center px-3 py-2 rounded-md text-vrc-text transition focus-within:bg-vrc-button/90 focus-within:border-vrc-highlight/70 hover:border-vrc-highlight/40"
      :class="$attrs.disabled ? 'opacity-60' : ''"
  >
    <div class="text-vrc-subtext">
      <slot/>
    </div>

    <input
        class="bg-transparent grow min-w-0 outline-none text-sm placeholder:text-vrc-subtext/80"
        v-bind="$attrs"
        :type="inputType"
    />

    <button
        v-if="isPassword"
        type="button"
        class="cursor-pointer px-2 text-vrc-subtext transition hover:text-vrc-highlight"
        @click="togglePassword"
        :aria-label="showPassword ? 'Hide password' : 'Show password'"
    >
      <EyeOffIcon v-if="showPassword"/>
      <EyeIcon v-else/>
    </button>
  </div>
</template>

