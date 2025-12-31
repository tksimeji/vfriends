import {invoke} from '@tauri-apps/api/core';
import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, onBeforeUnmount, onMounted, reactive, shallowRef, toRefs} from 'vue';

export type UseLoginFlowOptions = {
  onLoginSuccess?: (user: AuthUser | null) => void;
};

export type LoginState = 'credentials' | 'twoFactor' | 'success';
export type TwoFactorMethod = 'totp' | 'emailOtp' | 'otp';
export type AuthAction = 'credentials' | 'twoFactor';

export type AuthUser = {
  id: string;
  displayName: string;
  username?: string | null;
};

type AuthState = {
  username: string;
  password: string;
  twoFactorCode: string;
  twoFactorMethods: TwoFactorMethod[];
  selectedTwoFactorMethod: TwoFactorMethod | '';
  isSubmitting: boolean;
  errorMessage: string;
  successMessage: string;
  authedUser: AuthUser | null;
  currentStep: LoginState;
  activeAction: AuthAction | null;
};

type AuthEvent =
  | { type: 'started'; action: AuthAction }
  | { type: 'twoFactorRequired'; methods?: string[]; message?: string }
  | { type: 'success'; user?: AuthUser }
  | { type: 'failure'; message: string; code?: string }
  | { type: 'loggedOut' };

const KNOWN_2FA_METHODS: TwoFactorMethod[] = ['totp', 'emailOtp', 'otp'];
const methodSort = (a: TwoFactorMethod, b: TwoFactorMethod) =>
  KNOWN_2FA_METHODS.indexOf(a) - KNOWN_2FA_METHODS.indexOf(b);

const normalizeMethods = (methods: string[] | undefined) => {
  const filtered = (methods ?? []).filter((method): method is TwoFactorMethod =>
    KNOWN_2FA_METHODS.includes(method as TwoFactorMethod),
  );
  return Array.from(new Set(filtered)).sort(methodSort);
};

const createInitialState = (): AuthState => ({
  username: '',
  password: '',
  twoFactorCode: '',
  twoFactorMethods: [],
  selectedTwoFactorMethod: '',
  isSubmitting: false,
  errorMessage: '',
  successMessage: '',
  authedUser: null,
  currentStep: 'credentials',
  activeAction: null,
});

export const useAuthFlow = (options: UseLoginFlowOptions = {}) => {
  const state = reactive<AuthState>(createInitialState());
  const unlistenRef = shallowRef<UnlistenFn | null>(null);

  const resetTwoFactor = () => {
    state.twoFactorCode = '';
    state.twoFactorMethods = [];
    state.selectedTwoFactorMethod = '';
  };

  const resetForm = () => {
    resetTwoFactor();
    state.isSubmitting = false;
    state.errorMessage = '';
    state.successMessage = '';
    state.authedUser = null;
    state.currentStep = 'credentials';
    state.activeAction = null;
  };

  const clearStatus = () => {
    state.errorMessage = '';
    state.successMessage = '';
  };

  const markSubmitting = (action: AuthAction) => {
    state.isSubmitting = true;
    clearStatus();
    state.activeAction = action;
  };

  const markIdle = () => {
    state.isSubmitting = false;
    state.activeAction = null;
  };

  const failWith = (message: string) => {
    state.isSubmitting = false;
    state.errorMessage = message || '認証に失敗しました。';
    state.successMessage = '';
    state.activeAction = null;
  };

  const handleAuthEvent = (event: AuthEvent) => {
    switch (event.type) {
      case 'started':
        markSubmitting(event.action);
        return;
      case 'twoFactorRequired': {
        state.isSubmitting = false;
        state.twoFactorMethods = normalizeMethods(event.methods);
        state.selectedTwoFactorMethod = state.twoFactorMethods[0] ?? '';
        state.currentStep = 'twoFactor';
        state.errorMessage = state.twoFactorMethods.length === 0
          ? '未対応の2FA方式です。'
          : '';
        state.successMessage = '';
        state.activeAction = null;
        return;
      }
      case 'success': {
        state.isSubmitting = false;
        state.successMessage = 'ログインに成功しました。';
        state.currentStep = 'success';
        state.authedUser = event.user ?? {
          id: '',
          displayName: state.username || 'VRChat User',
          username: state.username || null,
        };
        state.password = '';
        state.twoFactorCode = '';
        state.activeAction = null;
        options.onLoginSuccess?.(state.authedUser);
        return;
      }
      case 'failure':
        failWith(event.message);
        return;
      case 'loggedOut':
        markIdle();
        resetForm();
        return;
    }
  };

  const currentTwoFactorMethod = computed(() => {
    return state.selectedTwoFactorMethod || state.twoFactorMethods[0] || '';
  });

  const handleCredentialsSubmit = async () => {
    if (state.isSubmitting) return;
    clearStatus();

    const trimmedUsername = state.username.trim();
    if (!trimmedUsername || !state.password) {
      state.errorMessage = 'ユーザー名とパスワードを入力してください。';
      return;
    }

    markSubmitting('credentials');
    try {
      await invoke('begin_auth', {
        username: trimmedUsername,
        password: state.password,
      });
    } catch (error) {
      markIdle();
      state.errorMessage = 'ログインに失敗しました。通信状態を確認してください。';
      console.error(error);
    }
  };

  const handleTwoFactorSubmit = async () => {
    if (state.isSubmitting) return;
    clearStatus();

    if (!state.twoFactorCode) {
      state.errorMessage = '2FAコードを入力してください。';
      return;
    }
    const method = currentTwoFactorMethod.value;
    if (!method) {
      state.errorMessage = '2FA方式を選択してください。';
      return;
    }

    markSubmitting('twoFactor');
    try {
      await invoke('verify_two_factor', {
        twoFactorCode: state.twoFactorCode.trim(),
        twoFactorMethod: method,
      });
    } catch (error) {
      markIdle();
      state.errorMessage = '2FAの検証に失敗しました。もう一度お試しください。';
      console.error(error);
    }
  };

  const handleBackToCredentials = () => {
    resetTwoFactor();
    clearStatus();
    state.currentStep = 'credentials';
    state.activeAction = null;
  };

  const handleSuccessClose = () => {
    resetForm();
  };

  const handleLogout = () => {
    resetForm();
  };

  onMounted(async () => {
    unlistenRef.value = await listen<AuthEvent>('vrc:auth', (event) => {
      handleAuthEvent(event.payload);
    });
    try {
      await invoke('restore_session');
    } catch (error) {
      console.error(error);
    }
  });

  onBeforeUnmount(() => {
    unlistenRef.value?.();
  });

  const {
    username,
    password,
    twoFactorCode,
    twoFactorMethods,
    selectedTwoFactorMethod,
    isSubmitting,
    errorMessage,
    successMessage,
    authedUser,
    currentStep,
    activeAction,
  } = toRefs(state);

  return {
    username,
    password,
    twoFactorCode,
    twoFactorMethods,
    selectedTwoFactorMethod,
    isSubmitting,
    errorMessage,
    successMessage,
    authedUser,
    currentStep,
    activeAction,
    currentTwoFactorMethod,
    handleCredentialsSubmit,
    handleTwoFactorSubmit,
    handleBackToCredentials,
    handleSuccessClose,
    handleLogout,
  };
};
