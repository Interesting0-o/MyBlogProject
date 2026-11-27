use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use super::error::ApiError;
use super::state::AppState;
use crate::domain::post::{Post, PostStatus};
use crate::service::post as post_service;

pub(super) async fn list(State(state): State<AppState>) -> Result<Json<PostsResponse>, ApiError> {
    let posts = post_service::list_posts(state.db.as_ref(), Some(PostStatus::Published)).await?;

    Ok(Json(PostsResponse {
        posts: posts.into_iter().map(PostResponse::from).collect(),
    }))
}

pub(super) async fn get(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = post_service::get_published_post_with_view(state.db.as_ref(), &slug).await?;
    Ok(Json(PostResponse::from(post)))
}

#[derive(Serialize)]
pub(super) struct PostsResponse {
    pub(super) posts: Vec<PostResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PostResponse {
    slug: String,
    title: String,
    description: String,
    published_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
    tags: Vec<String>,
    reading_minutes: u32,
    featured: bool,
    status: String,
    views: u32,
    content: String,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            slug: post.slug,
            title: post.title,
            description: post.description,
            published_at: post.published_at,
            updated_at: post.updated_at,
            tags: post.tags,
            reading_minutes: post.reading_minutes,
            featured: post.featured,
            status: post.status.as_str().to_string(),
            views: post.views,
            content: post.content,
        }
    }
}
