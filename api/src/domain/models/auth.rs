use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub identifier: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub user_uuid: Uuid,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user_uuid: Uuid,
    pub display_name: String,
    pub identifier: String,
}

