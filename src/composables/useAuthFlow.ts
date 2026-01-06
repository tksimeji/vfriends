import {listen, type UnlistenFn} from '@tauri-apps/api/event';
import {computed, onBeforeUnmount, onMounted, reactive, shallowRef, toRefs} from 'vue';
import {t} from '../i18n';
import {beginAuth, restoreSession, verifyTwoFactor} from '../invokes';
import {VRChat} from '../vrchat.ts';
import {useAuthSession, type AuthAction, type AuthEvent} from './useAuthSession';

const KNOWN_2FA_METHODS: TwoFactorMethod[] = ['totp', 'emailOtp', 'otp'];

export type UseLoginFlowOptions = {
  onLoginSuccess?: (user: VRChat.CurrentUser | null) => void;
};

export type LoginState = 'credentials' | 'twoFactor' | 'success';
export type TwoFactorMethod = 'totp' | 'emailOtp' | 'otp';
type AuthState = {
  username: string;
  password: string;
  twoFactorCode: string;
  twoFactorMethods: TwoFactorMethod[];
  selectedTwoFactorMethod: TwoFactorMethod | '';
  isSubmitting: boolean;
  errorMessage: string;
  successMessage: string;
  authedUser: VRChat.CurrentUser | null;
  currentStep: LoginState;
  activeAction: AuthAction | null;
};

const methodSort = (a: TwoFactorMethod, b: TwoFactorMethod) =>
  KNOWN_2FA_METHODS.indexOf(a) - KNOWN_2FA_METHODS.indexOf(b);

const normalizeMethods = (methods: string[] | undefined) => {
  const filtered = (methods ?? []).filter((method): method is TwoFactorMethod =>
    KNOWN_2FA_METHODS.includes(method as TwoFactorMethod),
  );
  return Array.from(new Set(filtered)).sort(methodSort);
};

export const useAuthFlow = (options: UseLoginFlowOptions = {}) => {
  const {setCurrentUser, clearCurrentUser, startLoginCelebration} = useAuthSession();
  const state = reactive<AuthState>({
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
    state.errorMessage = message || t('auth.errors.authFailed');
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
          ? t('auth.errors.unsupported2fa')
          : '';
        state.successMessage = '';
        state.activeAction = null;
        return;
      }
      case 'success': {
        state.isSubmitting = false;
        state.successMessage = t('auth.errors.loginSuccess');
        state.currentStep = 'success';
        state.authedUser = event.user ?? {
          id: '',
          displayName: state.username || t('auth.vrchatUserFallback'),
          username: state.username || undefined,
          currentAvatarImageUrl: '',
          profilePicOverride: '',
          userIcon: '',
        };
        state.password = '';
        state.twoFactorCode = '';
        state.activeAction = null;
        setCurrentUser(state.authedUser);
        startLoginCelebration();
        options.onLoginSuccess?.(state.authedUser);
        return;
      }
      case 'failure':
        failWith(event.message);
        return;
      case 'loggedOut':
        markIdle();
        resetForm();
        clearCurrentUser();
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
      state.errorMessage = t('auth.errors.missingCredentials');
      return;
    }

    markSubmitting('credentials');
    try {
      await beginAuth(trimmedUsername, state.password);
    } catch (error) {
      markIdle();
      state.errorMessage = t('auth.errors.loginFailedNetwork');
      console.error(error);
    }
  };

  const handleTwoFactorSubmit = async () => {
    if (state.isSubmitting) return;
    clearStatus();

    if (!state.twoFactorCode) {
      state.errorMessage = t('auth.errors.missingTwoFactor');
      return;
    }
    const method = currentTwoFactorMethod.value;
    if (!method) {
      state.errorMessage = t('auth.errors.selectTwoFactor');
      return;
    }

    markSubmitting('twoFactor');
    try {
      await verifyTwoFactor(state.twoFactorCode.trim(), method);
    } catch (error) {
      markIdle();
      state.errorMessage = t('auth.errors.verifyFailed');
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
    clearCurrentUser();
  };

  onMounted(async () => {
    unlistenRef.value = await listen<AuthEvent>('vrc:auth', (event) => {
      handleAuthEvent(event.payload);
    });
    try {
      await restoreSession();
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
