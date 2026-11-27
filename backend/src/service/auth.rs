use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::CONFIG;

const MAX_FAILURES: u32 = 5;
const FAILURE_WINDOW: Duration = Duration::from_secs(15 * 60);
const LOCK_DURATION: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("account temporarily locked")]
    AccountLocked,
    #[error("admin auth is not configured")]
    NotConfigured,
    #[error("token error: {0}")]
    Token(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub struct AuthenticatedAdmin {
    pub username: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub token: String,
    pub expires_at: String,
}

struct LoginState {
    failures: u32,
    window_started: Instant,
    locked_until: Option<Instant>,
}

impl LoginState {
    fn new() -> Self {
        Self {
            failures: 0,
            window_started: Instant::now(),
            locked_until: None,
        }
    }
}

/// 登录锁定状态的载体：失败计数与锁定截止时间在请求之间共享。
#[derive(Clone)]
pub struct LoginGuard {
    login_state: std::sync::Arc<Mutex<LoginState>>,
}

impl LoginGuard {
    pub fn new() -> Self {
        Self {
            login_state: std::sync::Arc::new(Mutex::new(LoginState::new())),
        }
    }
}

impl Default for LoginGuard {
    fn default() -> Self {
        Self::new()
    }
}

// 登录与校验逻辑为自由函数，状态通过参数注入。
pub async fn login(
    guard: &LoginGuard,
    username: &str,
    password: &str,
) -> Result<TokenResponse, AuthError> {
    let expected_username = CONFIG.admin_username.as_str();
    let Some(password_hash) = CONFIG.admin_password_hash.clone() else {
        return Err(AuthError::NotConfigured);
    };
    let Some(secret) = CONFIG.admin_jwt_secret.clone() else {
        return Err(AuthError::NotConfigured);
    };

    if is_locked(guard) {
        tracing::warn!(username = %username, "login attempt while account is locked");
        return Err(AuthError::AccountLocked);
    }

    let username_matches = constant_time_eq(username.trim(), expected_username);
    let password_matches = bcrypt::verify(password, &password_hash).unwrap_or(false);

    if !username_matches || !password_matches {
        record_failure(guard);
        tracing::warn!(username = %username, "failed admin login attempt");
        return Err(AuthError::InvalidCredentials);
    }

    reset_failures(guard);

    let token = issue_token(expected_username, &secret)?;
    tracing::info!(username = %expected_username, "admin login succeeded");
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<AuthenticatedAdmin, AuthError> {
    let Some(secret) = CONFIG.admin_jwt_secret.clone() else {
        return Err(AuthError::NotConfigured);
    };

    let mut validation = Validation::default();
    validation.validate_exp = true;

    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(map_jwt_error)?;

    Ok(AuthenticatedAdmin {
        username: decoded.claims.sub,
    })
}

pub(crate) fn issue_token(username: &str, secret: &str) -> Result<TokenResponse, AuthError> {
    let expires_in_secs =
        u64::try_from(CONFIG.admin_token_hours.max(1) * 3600).unwrap_or(12 * 3600);
    let exp = now_unix() + expires_in_secs;
    let claims = Claims {
        sub: username.to_string(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| AuthError::Token(error.to_string()))?;

    Ok(TokenResponse {
        token,
        expires_at: format_unix(exp),
    })
}

fn is_locked(guard: &LoginGuard) -> bool {
    let mut state = guard
        .login_state
        .lock()
        .expect("admin login state poisoned");

    if let Some(locked_until) = state.locked_until {
        if Instant::now() < locked_until {
            return true;
        }

        state.locked_until = None;
        state.failures = 0;
        state.window_started = Instant::now();
    }

    if state.window_started.elapsed() > FAILURE_WINDOW {
        state.failures = 0;
        state.window_started = Instant::now();
    }

    false
}

fn record_failure(guard: &LoginGuard) {
    let mut state = guard
        .login_state
        .lock()
        .expect("admin login state poisoned");

    if state.window_started.elapsed() > FAILURE_WINDOW {
        state.failures = 0;
        state.window_started = Instant::now();
    }

    state.failures += 1;

    if state.failures >= MAX_FAILURES {
        state.locked_until = Some(Instant::now() + LOCK_DURATION);
        tracing::warn!(
            failures = state.failures,
            lock_seconds = LOCK_DURATION.as_secs(),
            "admin account locked after repeated failures"
        );
    }
}

fn reset_failures(guard: &LoginGuard) {
    let mut state = guard
        .login_state
        .lock()
        .expect("admin login state poisoned");
    state.failures = 0;
    state.locked_until = None;
    state.window_started = Instant::now();
}

fn map_jwt_error(error: JwtError) -> AuthError {
    match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::InvalidCredentials,
        _ => AuthError::Token(error.to_string()),
    }
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());

    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;

    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }

    diff == 0
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn format_unix(seconds: u64) -> String {
    let days = seconds / 86_400;
    let remainder = seconds % 86_400;
    let (year, month, day) = civil_from_days(i64::try_from(days).unwrap_or(0));
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        remainder / 3600,
        (remainder % 3600) / 60,
        remainder % 60
    )
}

// Howard Hinnant's civil_from_days algorithm.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };

    (year, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_time_eq_matches_std_semantics() {
        assert!(constant_time_eq("admin", "admin"));
        assert!(!constant_time_eq("admin", "Admin"));
        assert!(!constant_time_eq("admin", "adminx"));
        assert!(!constant_time_eq("adminx", "admin"));
        assert!(constant_time_eq("", ""));
    }

    #[test]
    fn civil_from_days_handles_epoch_and_leap_years() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
    }

    #[test]
    fn format_unix_produces_iso_timestamp() {
        let formatted = format_unix(86_400 * 19_723);
        assert_eq!(formatted, "2024-01-01T00:00:00Z");
    }
}
