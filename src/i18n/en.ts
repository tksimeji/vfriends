const en = {
  input: {
    hidePassword: 'Hide password',
    showPassword: 'Show password',
  },
  filePicker: {
    clear: 'Clear',
    noFileSelected: 'Not set',
  },
  titleBar: {
    search: 'Search friends here',
  },
  userStatus: {
    joinMe: 'Join Me',
    online: 'Online',
    askMe: 'Ask Me',
    doNotDisturb: 'Do Not Disturb',
    offline: 'Offline',
  },
  time: {
    minutesAgo: '{count}m ago',
    hoursAgo: '{count}h ago',
    daysAgo: '{count}d ago',
  },
  auth: {
    displayName: 'Display name',
    login: 'Log in',
    loginSubtitle: 'Log in to VRChat',
    loginSuccess: 'Login successful',
    loginSuccessDetail: 'You are now logged in to VRChat.',
    vrchatUserFallback: 'VRChat User',
    passwordPlaceholder: 'Password',
    userId: 'User ID',
    usernamePlaceholder: 'Username',
    usernameLabel: 'Username',
    copyrightNotice:
      'VRChat copyrights of VRChat Inc. and this app is not affiliated with VRChat Inc.',
    disclaimer:
      'Your credentials are only used for the VRChat API and are never sent to any other servers.',
    twoFactorTitle: '2FA Verification',
    twoFactorPrompt: 'Enter your 2FA code.',
    twoFactorCodePlaceholder: '2FA code',
    twoFactorMethod: {
      totpLabel: 'Authenticator app',
      emailLabel: 'Email',
      recoveryLabel: 'Recovery',
      totpHelp: 'Enter the code from your authenticator app.',
      emailHelp: 'Enter the code sent to your email.',
      recoveryHelp: 'Enter your recovery code.',
    },
    sessionChecking: 'Checking session...',
    back: 'Back',
    submit: 'Submit',
    success: {
      close: 'Close',
      logout: 'Log out',
    },
    errors: {
      authFailed: 'Authentication failed.',
      unsupported2fa: 'Unsupported 2FA method.',
      loginSuccess: 'Login successful.',
      missingCredentials: 'Enter your username and password.',
      loginFailedNetwork: 'Login failed. Please check your connection.',
      missingTwoFactor: 'Enter your 2FA code.',
      selectTwoFactor: 'Select a 2FA method.',
      verifyFailed: 'Failed to verify 2FA. Please try again.',
    },
  },
  friends: {
    errors: {
      fetchFailed: 'Failed to load friends.',
    },
    loading: 'Loading friends...',
    empty: 'No friends found.',
    searchEmpty: 'No results found.',
    count: '{count} friends',
    countFiltered: '{filtered} / {total} friends',
    clickCardHint: 'Click a card to open notification config',
    updating: 'Updating...',
    searchPlaceholder: 'Search friends...',
    lastOnline: 'Last online: {value}',
    location: {
      private: 'Private',
      traveling: 'Traveling',
      offline: 'Offline',
      web: 'Web',
    },
    contextMenu: {
      openSettings: 'Open settings',
      disableNotifications: 'Disable notifications',
      enableNotifications: 'Enable notifications',
      openProfile: 'Open VRChat profile',
      openInstance: 'Open instance in VRChat',
    },
  },
  settings: {
    noSettings: 'No config available.',
    accountTitle: 'Account config',
    vrchatUserFallback: 'VRChat User',
    actions: {
      logout: 'Log out',
      logoutCancel: 'Cancel',
      logoutConfirm: 'Log out',
      logoutConfirmDescription:
        'This will sign you out from VRChat on this app.',
      logoutConfirmTitle: 'Log out?',
    },
    languageTitle: 'Language',
    languageLabel: 'App language',
    languageHelper: 'Applies immediately. Restart not required.',
    languageOptions: {
      en: 'English',
      ja: 'Japanese',
    },
    notifications: {
      title: 'Notification config',
      description: 'These config apply when no per-friend override exists.',
      messageLabel: 'Notification message',
      messagePlaceholder: '{name} is online',
      soundLabel: 'Notification sound (file)',
      testSound: 'Test sound',
      defaultMessageTemplate: '{name} is online',
      errors: {
        loadFailed: 'Failed to load notification config.',
        saveFailed: 'Failed to save notification config.',
      },
    },
    sidebar: {
      appSettings: 'App config',
    },
    friend: {
      deliveryTitle: 'Notification delivery',
      deliveryOn:
        'Turn the toggle off to pause notifier temporarily.',
      deliveryOff: 'Notifications are currently paused.',
      customizeTitle: 'Customize notifier',
      customizeOn:
        'Custom notifier are enabled for this friend. Turn off to use defaults.',
      customizeOff:
        'Turn this on to customize notifier for this friend.',
      messageLabel: 'Notification message',
      messageHelper: '"%s" will be replaced with the display name.',
      soundLabel: 'Notification sound',
      previewSound: 'Preview Sound',
    },
  },
};

export default en;
