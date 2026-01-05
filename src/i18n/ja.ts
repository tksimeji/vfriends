const ja = {
  titleBar: {
    search: 'ここでフレンドを検索',
  },
  filePicker: {
    clear: '削除',
    noFileSelected: '未設定',
  },
  userStatus: {
    joinMe: 'だれでもおいで',
    online: 'オンライン',
    askMe: 'きいてみてね',
    doNotDisturb: '取り込み中',
    offline: 'オフライン',
  },
  input: {
    hidePassword: 'パスワードを隠す',
    showPassword: 'パスワードを表示',
  },
  auth: {
    displayName: '表示名',
    login: 'ログイン',
    loginSubtitle: 'VRChatにログイン',
    loginSuccess: 'ログイン成功',
    loginSuccessDetail: 'VRChatにログインしました。',
    vrchatUserFallback: 'VRChat User',
    passwordPlaceholder: 'パスワード',
    userId: 'ユーザーID',
    usernamePlaceholder: 'ユーザー名',
    usernameLabel: 'ユーザー名',
    copyrightNotice:
      'VRChat copyrights of VRChat Inc. and this app is not affiliated with VRChat Inc.',
    disclaimer:
      'ログイン情報は VRChat APIの利用にのみ使用され，その他のサーバーに送信されることはありません．',
    twoFactorTitle: '2FA確認',
    twoFactorPrompt: '2FAコードを入力してください。',
    twoFactorCodePlaceholder: '2FAコード',
    twoFactorMethod: {
      totpLabel: '認証アプリ',
      emailLabel: 'メール',
      recoveryLabel: 'リカバリー',
      totpHelp: '認証アプリのコードを入力してください。',
      emailHelp: 'メールで届いたコードを入力してください。',
      recoveryHelp: 'リカバリーコードを入力してください。',
    },
    back: '戻る',
    submit: '送信',
    success: {
      close: '閉じる',
      logout: 'ログアウト',
    },
    errors: {
      authFailed: '認証に失敗しました。',
      unsupported2fa: '未対応の2FA方式です。',
      loginSuccess: 'ログインに成功しました。',
      missingCredentials: 'ユーザー名とパスワードを入力してください。',
      loginFailedNetwork: 'ログインに失敗しました。通信状態を確認してください。',
      missingTwoFactor: '2FAコードを入力してください。',
      selectTwoFactor: '2FA方式を選択してください。',
      verifyFailed: '2FAの検証に失敗しました。もう一度お試しください。',
    },
  },
  friends: {
    errors: {
      fetchFailed: 'フレンドの取得に失敗しました。',
    },
    loading: 'フレンド一覧を読み込み中...',
    empty: 'フレンドが見つかりません。',
    searchEmpty: '検索結果が見つかりません。',
    count: '{count}件',
    countFiltered: '{filtered} / {total}件',
    clickCardHint: 'カードをクリックで通知設定',
    updating: '更新中...',
    searchPlaceholder: 'フレンド検索...',
    lastOnline: '最終オンライン：{value}',
    location: {
      private: 'プライベート',
      traveling: '移動中',
      offline: 'オフライン',
      web: 'Web',
    },
  },
  settings: {
    noSettings: 'ここに表示できる設定がありません。',
    accountTitle: 'アカウント設定',
    vrchatUserFallback: 'VRChat User',
    actions: {
      logout: 'ログアウトする',
    },
    languageTitle: '言語',
    languageLabel: 'アプリの言語',
    languageHelper: '変更はすぐに反映されます。',
    languageOptions: {
      en: 'English',
      ja: '日本語',
    },
    notifications: {
      title: '通知設定',
      description:
        'フレンドに個別の設定を行わない場合、この設定が適用されます。',
      messageLabel: '通知メッセージ',
      messagePlaceholder: '{name} がオンラインになりました',
      soundLabel: '通知音（ファイル選択）',
      testSound: 'テスト再生',
      defaultMessageTemplate: '{name} がオンラインになりました',
      errors: {
        loadFailed: '通知設定の読み込みに失敗しました。',
        saveFailed: '通知設定の保存に失敗しました。',
      },
    },
    sidebar: {
      appSettings: 'アプリ設定',
    },
    friend: {
      deliveryTitle: '通知の配信',
      deliveryOn:
        'トグルスイッチをオフにすると，一時的に通知の配信を停止することができます．',
      deliveryOff: '現在，通知の配信を停止しています．',
      customizeTitle: '通知のカスタマイズ',
      customizeOn:
        'このフレンドだけの特別な通知を使用中．トグルスイッチをオフにすると，デフォルトの設定が使用されます．',
      customizeOff:
        'トグルスイッチをオンにして，このフレンドからの通知を特別なものにできます:)',
      messageLabel: '通知メッセージ',
      messageHelper: '「%s」は表示名に置き換えられます。',
      soundLabel: '通知サウンド',
      previewSound: 'サウンドをプレビュー',
    },
  },
  time: {
    minutesAgo: '{count}分前',
    hoursAgo: '{count}時間前',
    daysAgo: '{count}日前',
  },
};

export default ja;
