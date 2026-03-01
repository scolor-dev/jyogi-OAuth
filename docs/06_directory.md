# 📁 ディレクトリ構成図（部内OAuth基盤 / OIDC Provider版）

------------------------------------------------------------------------

# 0️⃣ 設計前提

  項目             内容
  ---------------- ----------------------------------
  リポジトリ構成   Monorepo
  アーキテクチャ   Clean Architecture + Layered
  デプロイ単位     単一Authサービス（将来分離可能）
  言語             Rust（axum + sqlx）想定
  MVP方針          P0に必要なディレクトリのみ

------------------------------------------------------------------------

# 1️⃣ 全体構成（Monorepo）

    root/
    ├── apps/
    │   ├── auth-api/        # OAuth / OIDC Provider 本体
    │   └── admin-web/       # 管理UI（任意）
    ├── packages/
    │   ├── domain/          # 共通ドメイン定義（User, Role等）
    │   ├── config/
    │   └── utils/
    ├── infra/
    │   ├── docker/
    │   └── ci/
    ├── docs/
    └── README.md

------------------------------------------------------------------------

# 2️⃣ バックエンド構成（auth-api）

    apps/auth-api/
    ├── src/
    │   ├── main.rs
    │   ├── router.rs
    │   ├── config/
    │   ├── middleware/
    │   │   ├── auth.rs
    │   │   └── logging.rs
    │   ├── domain/
    │   │   ├── user.rs
    │   │   ├── role.rs
    │   │   ├── oauth_client.rs
    │   │   └── token.rs
    │   ├── usecase/
    │   │   ├── authorize.rs
    │   │   ├── issue_token.rs
    │   │   ├── refresh_token.rs
    │   │   └── user_management.rs
    │   ├── repository/
    │   │   ├── user_repo.rs
    │   │   ├── client_repo.rs
    │   │   └── token_repo.rs
    │   ├── handler/
    │   │   ├── oauth.rs
    │   │   ├── user.rs
    │   │   └── admin.rs
    │   └── jwk/
    │       ├── key_manager.rs
    │       └── jwks.rs
    ├── migrations/
    └── tests/

------------------------------------------------------------------------

# 3️⃣ DBマイグレーション構成

    migrations/
    ├── 0001_create_users.sql
    ├── 0002_create_roles.sql
    ├── 0003_create_identities.sql
    ├── 0004_create_oauth_clients.sql
    ├── 0005_create_authorization_codes.sql
    ├── 0006_create_refresh_tokens.sql
    └── 0007_create_audit_logs.sql

------------------------------------------------------------------------

# 4️⃣ インフラ構成

    infra/
    ├── docker/
    │   ├── Dockerfile.auth
    │   └── docker-compose.yml
    ├── ci/
    │   └── github-actions.yml

------------------------------------------------------------------------

# 5️⃣ テスト構成

    tests/
    ├── unit/
    ├── integration/
    └── fixtures/

------------------------------------------------------------------------

# 設計思想まとめ

-   OAuth/OIDCエンドポイントと管理APIを責務分離
-   ドメイン層とインフラ層を明確分離
-   users.uuid を外部公開IDとして統一
-   JWT署名鍵管理を専用モジュール化
