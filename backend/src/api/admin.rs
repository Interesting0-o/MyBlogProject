use axum::{
    Json,
    extract::{Path, Query, State},
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::error::ApiError;
use super::middleware::ADMIN_COOKIE;
use super::posts::{PostResponse, PostsResponse};
use super::state::AppState;
use crate::domain::comment::{Comment, CommentStatus};
use crate::domain::post::{Post, PostStatus};
use crate::service::setting::SiteSettings;
use crate::service::stats::StatsSummary;

// ---------- auth ----------

#[derive(Deserialize)]
pub(super) struct LoginRequest {
    username: String,
    password: String,
}

pub(super) async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Response, ApiError> {
    let token = state
        .auth
        .login(&request.username, &request.password)
        .await?;

    let cookie = format!(
        "{}={}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}",
        ADMIN_COOKIE,
        token.token,
        max_age_seconds()
    );

    Ok((
        [(SET_COOKIE, cookie)],
        Json(json!({
            "token": token.token,
            "expiresAt": token.expires_at,
            "username": request.username.trim(),
        })),
    )
        .into_response())
}

fn max_age_seconds() -> i64 {
    crate::config::CONFIG.admin_token_hours.max(1) * 3600
}

pub(super) async fn logout() -> Response {
    let cookie = format!("{ADMIN_COOKIE}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0");
    ([(SET_COOKIE, cookie)], StatusCode::NO_CONTENT).into_response()
}

pub(super) async fn session(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    // 由 require_admin 中间件保证已通过认证。
    let _ = state;
    Ok(Json(json!({ "authenticated": true })))
}

// ---------- posts ----------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PostRequest {
    slug: String,
    title: String,
    description: String,
    published_at: String,
    updated_at: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    reading_minutes: u32,
    #[serde(default)]
    featured: bool,
    status: Option<String>,
    content: String,
}

impl TryFrom<PostRequest> for Post {
    type Error = crate::service::post::ServiceError;

    fn try_from(request: PostRequest) -> Result<Self, Self::Error> {
        let status = match request.status.as_deref() {
            None => PostStatus::Draft,
            Some(status) => PostStatus::parse(status).ok_or_else(|| {
                crate::service::post::ServiceError::Validation(format!("invalid status: {status}"))
            })?,
        };

        Ok(Self {
            slug: request.slug,
            title: request.title,
            description: request.description,
            published_at: request.published_at,
            updated_at: request.updated_at,
            tags: request.tags,
            reading_minutes: request.reading_minutes,
            featured: request.featured,
            status,
            views: 0,
            content: request.content,
        })
    }
}

pub(super) async fn list_posts(
    State(state): State<AppState>,
) -> Result<Json<PostsResponse>, ApiError> {
    let posts = state.posts.list_posts(None).await?;

    Ok(Json(PostsResponse {
        posts: posts.into_iter().map(PostResponse::from).collect(),
    }))
}

pub(super) async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = state.posts.get_post(&slug, None).await?;
    Ok(Json(PostResponse::from(post)))
}

pub(super) async fn create_post(
    State(state): State<AppState>,
    Json(request): Json<PostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = state.posts.create_post(Post::try_from(request)?).await?;
    Ok(Json(PostResponse::from(post)))
}

pub(super) async fn update_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(request): Json<PostRequest>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = state
        .posts
        .update_post(&slug, Post::try_from(request)?)
        .await?;
    Ok(Json(PostResponse::from(post)))
}

pub(super) async fn delete_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.posts.delete_post(&slug).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------- comments ----------

#[derive(Deserialize)]
pub(super) struct CommentStatusQuery {
    status: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CommentResponse {
    id: i64,
    post_slug: String,
    author: String,
    content: String,
    created_at: String,
    status: String,
}

impl From<Comment> for CommentResponse {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id,
            post_slug: comment.post_slug,
            author: comment.author,
            content: comment.content,
            created_at: comment.created_at,
            status: comment.status.as_str().to_string(),
        }
    }
}

#[derive(Serialize)]
pub(super) struct CommentsResponse {
    pub(super) comments: Vec<CommentResponse>,
}

pub(super) async fn list_comments(
    State(state): State<AppState>,
    Query(query): Query<CommentStatusQuery>,
) -> Result<Json<CommentsResponse>, ApiError> {
    let status = match query.status.as_deref() {
        None | Some("all") => None,
        Some(value) => Some(
            CommentStatus::parse(value)
                .ok_or_else(|| ApiError::Validation(format!("invalid status: {value}")))?,
        ),
    };

    let comments = state.comments.list(status).await?;

    Ok(Json(CommentsResponse {
        comments: comments.into_iter().map(CommentResponse::from).collect(),
    }))
}

#[derive(Deserialize)]
pub(super) struct CommentStatusRequest {
    status: String,
}

pub(super) async fn set_comment_status(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(request): Json<CommentStatusRequest>,
) -> Result<Json<CommentResponse>, ApiError> {
    let status = CommentStatus::parse(&request.status)
        .ok_or_else(|| ApiError::Validation(format!("invalid status: {}", request.status)))?;

    state.comments.set_status(id, status).await?;

    let comment = state
        .comments
        .find(id)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(CommentResponse::from(comment)))
}

pub(super) async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state.comments.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ---------- settings ----------

pub(super) async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<SiteSettings>, ApiError> {
    let settings = state.settings.get().await?;
    Ok(Json(settings))
}

pub(super) async fn update_settings(
    State(state): State<AppState>,
    Json(settings): Json<SiteSettings>,
) -> Result<Json<SiteSettings>, ApiError> {
    let settings = state.settings.update(&settings).await?;
    Ok(Json(settings))
}

// ---------- stats ----------

pub(super) async fn stats(State(state): State<AppState>) -> Result<Json<StatsSummary>, ApiError> {
    let summary = state.stats.summary().await?;
    Ok(Json(summary))
}
