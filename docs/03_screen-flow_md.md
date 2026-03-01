# 🖥️ screen-flow.md テンプレート

---

# 0️⃣ 設計前提

| 項目     | 内容                             |
| ------ | ------------------------------ |
| 対象ユーザー | 未ログイン / 部員（member）/ 管理者（admin） |
| デバイス   | Desktop中心（管理UI前提）/ Responsive  |
| 認証要否   | 基本全面認証制（/loginのみ公開）            |
| 権限制御   | RBAC（admin / member）           |
| MVP範囲  | P0画面のみ（OIDC成立に必要なUI）           |

---

# 1️⃣ 画面一覧（Screen Inventory）

| ID   | 画面名        | 役割             | 認証  | 優先度 |
| ---- | ---------- | -------------- | --- | --- |
| S-01 | ログイン選択     | Discordログイン入口  | 不要  | P0  |
| S-02 | 認可確認画面     | OAuthスコープ同意    | 必須  | P0  |
| S-03 | ダッシュボード    | ユーザー中心画面       | 必須  | P0  |
| S-04 | プロフィール     | 自分の情報確認        | 必須  | P0  |
| S-05 | Identity管理 | 外部連携確認         | 必須  | P1  |
| S-06 | クライアント管理   | OAuthアプリ登録     | 管理者 | P0  |
| S-07 | ユーザー管理     | 部員管理           | 管理者 | P1  |
| S-08 | ロール管理      | admin/member制御 | 管理者 | P1  |
| S-09 | 監査ログ       | ログイン履歴確認       | 管理者 | P2  |

---

# 2️⃣ 全体遷移図（高レベル）

```mermaid
flowchart TD
    LOGIN[Login]
    CONSENT[Consent]
    DASH[Dashboard]
    PROFILE[Profile]
    IDENT[Identity]
    CLIENT[OAuth Clients]
    USERADMIN[User Admin]

    LOGIN --> DASH
    DASH --> PROFILE
    DASH --> IDENT
    DASH --> CLIENT
    DASH --> USERADMIN
    CLIENT --> CONSENT
```

---

# 3️⃣ 認証フロー

```mermaid
flowchart LR
    App[部内アプリ]
    Authorize[/authorize]
    Login[Login]
    Consent[Consent]
    Token[/token]
    AppHome[アプリ画面]

    App --> Authorize
    Authorize --> Login
    Login --> Consent
    Consent --> Token
    Token --> AppHome
```

---

# 4️⃣ CRUD標準遷移テンプレ

```mermaid
flowchart LR
    UserList --> UserDetail
    UserDetail --> EditUser
    ClientList --> CreateClient
    CreateClient --> ClientDetail
```

---

# 5️⃣ 状態別分岐（State-based Flow）

```mermaid
flowchart TD
    Login --> StatusCheck{user_status}
    StatusCheck -->|pending| PendingScreen
    StatusCheck -->|active| Dashboard
    StatusCheck -->|suspended| SuspendedNotice
    StatusCheck -->|banned| AccessDenied
```

---

# 6️⃣ 権限別分岐（RBAC/ABAC）

```mermaid
flowchart TD
    Dashboard --> RoleCheck{Role}
    RoleCheck -->|member| Profile
    RoleCheck -->|admin| AdminPanel
```

---

# 7️⃣ モーダル・非同期操作

```mermaid
flowchart LR
    ClientDetail --> RotateSecretModal
    RotateSecretModal --> Confirm
    Confirm --> SecretUpdatedToast
```

---

# 8️⃣ エラーフロー

```mermaid
flowchart TD
    Authorize --> ValidateClient
    ValidateClient -->|Invalid| ErrorPage
    ValidateClient -->|Valid| IssueCode
    IssueCode --> Token
    Token -->|InvalidCode| OAuthError
    Token -->|Success| ReturnToApp
```

---

# 9️⃣ 空状態 / 初回体験

```mermaid
flowchart TD
    Dashboard --> HasIdentity{Discord連携済?}
    HasIdentity -->|No| ConnectDiscord
    HasIdentity -->|Yes| NormalView
```

---

# 🔟 モバイル考慮（任意）

| 項目      | Desktop | Mobile     |
| ------- | ------- | ---------- |
| ナビゲーション  | Sidebar  | Drawer |
| クライアント管理 | テーブル表示   | 縦カード   |
| 認可画面     | 詳細スコープ表示 | 簡易表示   |


---

# 12️⃣ URL設計テンプレ（OAuth）

```
/login
/dashboard
/profile
/identities
/admin/users
/admin/clients
/admin/roles

/oauth/authorize
/oauth/token
/oauth/consent
/.well-known/openid-configuration
/.well-known/jwks.json
```
