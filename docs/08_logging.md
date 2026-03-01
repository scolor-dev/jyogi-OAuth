# 📘 ログ設計（部内OAuth基盤 / OIDC Provider版）

------------------------------------------------------------------------

# 0️⃣ 設計前提

  項目           内容
  -------------- ----------------------------------
  対象システム   Auth API（OIDC Provider）
  ログ方式       構造化ログ（JSON必須）
  集約方式       Centralized Logging
  保持期間       Audit 1年以上 / その他90日
  個人情報       トークン・秘密情報は絶対出力禁止

------------------------------------------------------------------------

# 1️⃣ ログ分類

  種別              目的                 出力対象
  ----------------- -------------------- --------------
  Application Log   バグ解析             開発・運用
  Access Log        API追跡              運用
  Audit Log         権限・設定変更追跡   セキュリティ
  Security Log      不正検知             SOC
  Auth Event Log    認証イベント記録     セキュリティ

------------------------------------------------------------------------

# 2️⃣ ログレベル

  レベル   用途
  -------- ----------------------
  DEBUG    詳細（本番無効推奨）
  INFO     正常動作
  WARN     想定内異常
  ERROR    処理失敗
  FATAL    起動不能

------------------------------------------------------------------------

# 3️⃣ 共通JSONフォーマット

``` json
{
  "timestamp": "2026-01-01T00:00:00Z",
  "level": "INFO",
  "service": "auth-api",
  "environment": "prod",
  "trace_id": "uuid",
  "user_uuid": "uuid",
  "client_id": "uuid",
  "action": "oauth.token.issue",
  "resource_type": "user",
  "resource_id": "uuid",
  "result": "success",
  "ip": "x.x.x.x",
  "user_agent": "...",
  "metadata": {}
}
```

------------------------------------------------------------------------

# 4️⃣ 必須フィールド

  フィールド   理由
  ------------ ------------------
  timestamp    時系列解析
  level        重要度
  service      サービス識別
  trace_id     分散トレーシング
  user_uuid    監査
  action       操作識別
  result       allow/deny

------------------------------------------------------------------------

# 5️⃣ OAuth特有ログ

## 5-1. 認可コード発行

``` json
{
  "action": "oauth.authorize",
  "client_id": "uuid",
  "user_uuid": "uuid",
  "scope": "openid profile",
  "result": "allow"
}
```

## 5-2. トークン発行

``` json
{
  "action": "oauth.token.issue",
  "client_id": "uuid",
  "user_uuid": "uuid",
  "grant_type": "authorization_code",
  "result": "success"
}
```

## 5-3. Refreshローテーション

``` json
{
  "action": "oauth.refresh.rotate",
  "user_uuid": "uuid",
  "client_id": "uuid",
  "result": "success"
}
```

------------------------------------------------------------------------

# 6️⃣ Audit Log（重要）

対象：

-   user_status変更
-   role付与/剥奪
-   client_secret再発行
-   redirect_uri変更

``` json
{
  "action": "role.grant",
  "resource_type": "user",
  "resource_id": "uuid",
  "before": {"role": "member"},
  "after": {"role": "admin"},
  "actor_user_uuid": "uuid",
  "result": "allow"
}
```

------------------------------------------------------------------------

# 7️⃣ Security Log

  イベント            記録
  ------------------- ------
  ログイン失敗        必須
  PKCE不一致          必須
  不正client_id       必須
  expired token使用   必須
  rate limit発動      推奨

------------------------------------------------------------------------

# 8️⃣ 分散トレーシング

-   trace_id必須
-   HTTP Header伝播
-   OpenTelemetry推奨

``` mermaid
flowchart LR
    Client --> CDN
    CDN --> AuthAPI
    AuthAPI --> Postgres
    AuthAPI --> Redis
```

------------------------------------------------------------------------

# 9️⃣ 保存構成

``` mermaid
flowchart LR
    AuthAPI --> LogAgent
    LogAgent --> LogCollector
    LogCollector --> LogStorage
    LogStorage --> Monitoring
```

------------------------------------------------------------------------

# 🔟 保持ポリシー

  種類          保持期間
  ------------- ----------
  Application   30日
  Access        90日
  Audit         1年以上
  Security      1年以上

------------------------------------------------------------------------

# 11️⃣ マスキングポリシー

  対象            方針
  --------------- ------------------
  client_secret   出力禁止
  access_token    出力禁止
  refresh_token   出力禁止
  email           ハッシュ化
  IP              必要に応じ匿名化

------------------------------------------------------------------------

# 12️⃣ フェーズ導入

Phase0: - Application Log - Access Log - OAuth主要イベントログ

Phase1: - Audit Log完全対応 - Centralized Logging

Phase2: - 分散トレーシング - セキュリティアラート自動化

Phase3: - SIEM連携 - 異常検知

------------------------------------------------------------------------

# 設計原則

-   OAuth基盤ではAudit Logが最重要
-   トークンは絶対にログ出力しない
-   user_status変更は必ず監査ログ
-   全APIでtrace_id必須
