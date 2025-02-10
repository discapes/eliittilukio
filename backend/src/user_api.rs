use crate::{
    app_error::AppError,
    util::{
        claims_from_cookie, insert_jwt_into_headers, GenericResponse, DEFAULT_COOKIE_OPTS, JWT_TTL,
    },
};
use anyhow::anyhow;
use axum::{
    extract::State,
    http::{header, HeaderMap},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use bigdecimal::ToPrimitive;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

#[derive(Deserialize)]
pub struct UpdateScorePayload {
    newscore: u32,
}

fn validate_email(email: &str) -> bool {
    email.ends_with("@edu.lieto.fi")
}

// typedheader must be before body
pub async fn update_score(
    State(db): State<SqlitePool>,
    cookie: CookieJar,
    Json(payload): Json<UpdateScorePayload>,
) -> Result<Json<GenericResponse>, AppError> {
    let claims = claims_from_cookie(cookie).map_err(AppError::Request)?;

    if sqlx::query!(
        "UPDATE users SET score = ?1 WHERE username = ?2 AND score <= ?1",
        payload.newscore,
        claims.sub,
    )
    .execute(&db)
    .await?
    .rows_affected()
        == 1
    {
        Ok(Json(GenericResponse {
            message: "Score updated",
        }))
    } else {
        Err(AppError::Request(anyhow::anyhow!(
            "Failed to update score: is it really your highscore?"
        )))
    }
}

pub async fn average_score(
    State(db): State<SqlitePool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let res = sqlx::query!("SELECT AVG(score) as average FROM users WHERE banned=0")
        .fetch_one(&db)
        .await?
        .average
        .unwrap_or(0)
        .to_u64();

    Ok(Json(json!({
            "average": res
    })))
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(
    State(db): State<SqlitePool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(HeaderMap, Json<GenericResponse>), AppError> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    if !validate_email(&payload.email) {
        return Err(AppError::Request(anyhow!("Error id 5 occured")));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| anyhow::anyhow!("Failed to create hash"))?
        .to_string();

    sqlx::query!(
        "INSERT INTO users (username, email, hash) VALUES (?, ?, ?)",
        payload.username,
        payload.email,
        password_hash,
    )
    .execute(&db)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                AppError::Request(anyhow!("Failed to create user: {}", db_err.message()))
            } else {
                AppError::Internal(anyhow!(db_err))
            }
        }
        _ => AppError::Internal(anyhow!(err)),
    })?;

    let mut headers = HeaderMap::new();
    insert_jwt_into_headers(&mut &mut headers, payload.username, payload.email, JWT_TTL);

    Ok((
        headers,
        Json(GenericResponse {
            message: "Created user",
        }),
    ))
}

pub async fn list_users(
    State(db): State<SqlitePool>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let list = sqlx::query!(
        "SELECT username, score FROM users WHERE banned = 0 ORDER BY score DESC LIMIT 10"
    )
    .fetch_all(&db)
    .await?
    .iter()
    .map(|rec| {
        json!({
            "username": rec.username,
            "score": rec.score
        })
    })
    .collect();
    Ok(Json(list))
}

pub async fn me(
    State(db): State<SqlitePool>,
    cookie: CookieJar,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = claims_from_cookie(cookie).map_err(AppError::Request)?;

    let rec = sqlx::query!("SELECT * FROM users WHERE username = ?", claims.sub)
        .fetch_one(&db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::Request(anyhow::anyhow!("Invalid credentials!")),
            ot => ot.into(),
        })?;

    Ok(Json(json!({
        "username": rec.username,
        "email": rec.email,
        "score": rec.score,
        "banned": rec.banned,
        "created": rec.created,
        "modified": rec.modified
    })))
}

pub async fn logout(State(_db): State<SqlitePool>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        format!("jwt=; expires=Thu, 01 Jan 1970 00:00:00 UTC; {DEFAULT_COOKIE_OPTS}",)
            .parse()
            .unwrap(),
    );
    headers
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login(
    State(db): State<SqlitePool>,
    Json(payload): Json<LoginPayload>,
) -> Result<(HeaderMap, Json<GenericResponse>), AppError> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let rec = sqlx::query!(
        "SELECT hash, username, email FROM users WHERE username = ?",
        payload.username,
    )
    .fetch_one(&db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::Request(anyhow::anyhow!("User doesn't exist!")),
        ot => ot.into(),
    })?;

    let parsed_hash =
        PasswordHash::new(&rec.hash).map_err(|_| anyhow::anyhow!("Failed to parse hash"))?;
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Request(anyhow::anyhow!("Invalid password")))?;

    let mut headers = HeaderMap::new();
    insert_jwt_into_headers(&mut &mut headers, rec.username, rec.email, JWT_TTL);

    Ok((
        headers,
        Json(GenericResponse {
            message: "Logged in",
        }),
    ))
}
