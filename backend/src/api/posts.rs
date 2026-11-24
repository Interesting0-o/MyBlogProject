use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use super::error::ApiError;
use crate::domain::post::Post;
use crate::service::post::PostService;

pub(super) async fn list(
    State(service): State<PostService>,
) -> Result<Json<PostsResponse>, ApiError> {
    let posts = service.list_posts().await?;

    Ok(Json(PostsResponse {
        posts: posts.into_iter().map(PostResponse::from).collect(),
    }))
}

pub(super) async fn get(
    State(service): State<PostService>,
    Path(slug): Path<String>,
) -> Result<Json<PostResponse>, ApiError> {
    let post = service.get_post(&slug).await?;
    Ok(Json(PostResponse::from(post)))
}

#[derive(Serialize)]
pub(super) struct PostsResponse {
    posts: Vec<PostResponse>,
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
            content: post.content,
        }
    }
}
