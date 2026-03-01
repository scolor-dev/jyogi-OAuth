# Infrastructure Guidelines（部内OAuth基盤 / OIDC Provider版）

------------------------------------------------------------------------

# System Architecture

``` mermaid
flowchart LR
    subgraph EDGE["Edge Layer"]
        CDN["CDN / TLS Termination"]
        WAF["WAF"]
    end

    subgraph APP["Application Layer"]
        LB["Load Balancer"]
        AUTH_APP["Auth API Service (axum)"]
        CACHE["Redis Cache"]
    end

    subgraph DATA["Data Layer"]
        POSTGRES["PostgreSQL (Primary)"]
        REPLICA["Read Replica"]
        LOGS["Log Storage"]
    end

    CDN --> WAF
    WAF --> LB
    LB --> AUTH_APP
    AUTH_APP --> CACHE
    AUTH_APP --> POSTGRES
    POSTGRES --> REPLICA
    AUTH_APP --> LOGS
```

------------------------------------------------------------------------

# System Components

## Edge Layer

### CDN

-   TLS終端
-   静的アセット配信
-   DDoS耐性

### WAF

-   レート制限
-   不正IP遮断

------------------------------------------------------------------------

## Application Layer

### Auth API Service

-   Rust (axum + tokio + sqlx)
-   OIDC Provider
-   JWT発行（RS256）
-   PKCE対応
-   RBAC + Scope判定

責務: - /oauth/authorize - /oauth/token - /.well-known/jwks.json -
/userinfo - 管理API

### Cache（Redis）

用途: - 認可コード短期保存 - レート制限 - 一時セッション情報

------------------------------------------------------------------------

## Data Layer

### PostgreSQL

用途: - users - identities - oauth_clients - authorization_codes -
refresh_tokens - audit_logs

設計方針: - RefreshTokenはハッシュ保存 - Read Replicaで読み取り分散

### Log Storage

-   監査ログ
-   認証イベントログ

------------------------------------------------------------------------

# スケーリング戦略

  レイヤー   方法
  ---------- ---------------------
  Edge       自動スケール
  App        コンテナAutoScaling
  DB         Read Replica
  Cache      Redis Cluster

------------------------------------------------------------------------

# セキュリティ方針

-   PKCE必須
-   state / nonce必須
-   RS256署名
-   KMS鍵管理
-   user_status最優先チェック

------------------------------------------------------------------------

# 将来拡張

-   HSM連携
-   Key Rotation自動化
-   mTLS
-   Zero Trust構成
