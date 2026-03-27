use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{
    state::AppState,
    domain::models::auth::{LoginRequest, LoginResponse, SignupRequest, SignupResponse, MeResponse},
    errors::api::ApiError,
    service,
};

/// POST /auth/signup — ユーザー登録
///
/// # Errors
/// - バリデーション失敗: 400
/// - identifier 重複: 409
/// - その他: 500
pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<(StatusCode, Json<SignupResponse>), ApiError> {
    let response = service::auth::signup(state.db(), &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /auth/login — パスワードログイン
///
/// # Errors
/// - 認証失敗: 401
/// - その他: 500
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), ApiError> {
    let (access_token, refresh_token) = service::auth::login(state.db(), &state, &body).await?;

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .path("/")
        .build();

    Ok((
        StatusCode::OK,
        jar.add(refresh_cookie),
        Json(LoginResponse { access_token }),
    ))
}

/// POST /auth/refresh — アクセストークン更新
///
/// # Errors
/// - トークン未送信・無効: 401
/// - その他: 500
pub async fn refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar, Json<LoginResponse>), ApiError> {
    let raw_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or(ApiError::Unauthorized)?;

    let (access_token, refresh_token) = service::auth::refresh(state.db(), &state, &raw_token).await?;

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .path("/")
        .build();

    Ok((
        StatusCode::OK,
        jar.add(refresh_cookie),
        Json(LoginResponse { access_token }),
    ))
}

/// POST /auth/logout — ログアウト
///
/// # Errors
/// - その他: 500
pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let raw_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or(ApiError::Unauthorized)?;

    service::auth::logout(state.db(), &raw_token).await?;

    let jar = jar.remove(Cookie::from("refresh_token"));

    Ok((jar, StatusCode::OK))
}

/// GET /auth/me — 認証済みユーザー情報取得
///
/// # Errors
/// - 未認証: 401
/// - その他: 500
pub async fn me(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<(StatusCode, Json<MeResponse>), ApiError> {
    let raw_token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized)?;

    let response = service::auth::me(state.db(), &state, raw_token).await?;
    Ok((StatusCode::OK, Json(response)))
}

