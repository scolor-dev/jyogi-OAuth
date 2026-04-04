pub fn validate_username(username: &str) -> Option<&'static str> {
    let s = username.trim();
    if s.len() < 3 || s.len() > 32 {
        return Some("username must be between 3 and 32 characters");
    }
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Some("username can only contain alphanumeric characters and underscores");
    }
    None
}

pub fn validate_password(password: &str) -> Option<&'static str> {
    let s = password.trim();
    if s.len() < 8 || s.len() > 72 {
        return Some("password must be between 8 and 72 characters");
    }
    None
}

pub fn validate_display_name(display_name: &str) -> Option<&'static str> {
    let s = display_name.trim();
    if s.is_empty() || s.len() > 50 {
        return Some("display_name must be between 1 and 50 characters");
    }
    None
}