use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

use super::admin::{CommentResponse, CommentsResponse};
use super::error::ApiError;
use super::state::AppState;

pub(super) async fn list_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<CommentsResponse>, ApiError> {
    let comments = state.comments.list_public(&slug).await?;

    Ok(Json(CommentsResponse {
        comments: comments.into_iter().map(CommentResponse::from).collect(),
    }))
}

#[derive(Deserialize)]
pub(super) struct NewCommentRequest {
    author: String,
    content: String,
}

pub(super) async fn create_comment(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(request): Json<NewCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), ApiError> {
    let comment = state
        .comments
        .create(&slug, &request.author, &request.content)
        .await?;

    Ok((StatusCode::CREATED, Json(CommentResponse::from(comment))))
}
