use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sudo4_api::{
    domain::post::{Post, PostStatus},
    infrastructure::sqlite::SqliteUnitOfWorkFactory,
    service::post::{PostService, ServiceError},
};

async fn test_service() -> PostService {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    PostService::new(Arc::new(SqliteUnitOfWorkFactory::new(pool)))
}

fn sample_post(slug: &str) -> Post {
    Post {
        slug: slug.to_string(),
        title: "Test post".to_string(),
        description: "A post used by the service tests".to_string(),
        published_at: "2026-11-24".to_string(),
        updated_at: None,
        tags: vec![
            "Rust".to_string(),
            "Rust".to_string(),
            "Backend".to_string(),
        ],
        reading_minutes: 4,
        featured: false,
        status: PostStatus::Published,
        views: 0,
        content: "<p>Hello</p>".to_string(),
    }
}

#[tokio::test]
async fn creates_reads_updates_and_deletes_posts() {
    let service = test_service().await;

    let created = service
        .create_post(sample_post("first-post"))
        .await
        .expect("create post");
    assert_eq!(
        created.tags,
        vec!["Rust".to_string(), "Backend".to_string()]
    );

    let posts = service
        .list_posts(None)
        .await
        .expect("list posts");
    assert!(posts.iter().any(|post| post.slug == "first-post"));

    let fetched = service
        .get_post("first-post", None)
        .await
        .expect("get post");
    assert_eq!(fetched.title, "Test post");

    let mut updated = fetched;
    updated.title = "Updated post".to_string();
    updated.updated_at = Some("2026-11-25".to_string());
    let updated = service
        .update_post("first-post", updated)
        .await
        .expect("update post");
    assert_eq!(updated.title, "Updated post");
    assert_eq!(updated.updated_at.as_deref(), Some("2026-11-25"));

    service
        .delete_post("first-post")
        .await
        .expect("delete post");
    assert!(matches!(
        service.get_post("first-post", None).await,
        Err(ServiceError::NotFound)
    ));
}

#[tokio::test]
async fn rejects_duplicate_slugs() {
    let service = test_service().await;

    service
        .create_post(sample_post("duplicate"))
        .await
        .expect("create post");

    assert!(matches!(
        service.create_post(sample_post("duplicate")).await,
        Err(ServiceError::Conflict(_))
    ));
}

#[tokio::test]
async fn renames_slug_on_update() {
    let service = test_service().await;

    service
        .create_post(sample_post("old-slug"))
        .await
        .expect("create post");

    let mut renamed = sample_post("new-slug");
    renamed.updated_at = Some("2026-11-25".to_string());
    service
        .update_post("old-slug", renamed)
        .await
        .expect("rename post");

    assert!(matches!(
        service.get_post("old-slug", None).await,
        Err(ServiceError::NotFound)
    ));
    let fetched = service
        .get_post("new-slug", None)
        .await
        .expect("get renamed post");
    assert_eq!(fetched.slug, "new-slug");
}

#[tokio::test]
async fn hides_drafts_from_public_listing() {
    let service = test_service().await;

    let mut draft = sample_post("draft-post");
    draft.status = PostStatus::Draft;
    service.create_post(draft).await.expect("create draft");
    service
        .create_post(sample_post("published-post"))
        .await
        .expect("create published post");

    let public_posts = service
        .list_posts(Some(PostStatus::Published))
        .await
        .expect("list published posts");
    assert!(public_posts.iter().all(|post| post.slug != "draft-post"));
    assert!(public_posts
        .iter()
        .any(|post| post.slug == "published-post"));

    let all_posts = service.list_posts(None).await.expect("list all posts");
    assert!(all_posts.iter().any(|post| post.slug == "draft-post"));
    assert_eq!(all_posts.len(), public_posts.len() + 1);

    assert!(matches!(
        service.get_post("draft-post", Some(PostStatus::Published)).await,
        Err(ServiceError::NotFound)
    ));
    let draft_fetched = service
        .get_post("draft-post", None)
        .await
        .expect("get draft post");
    assert_eq!(draft_fetched.status, PostStatus::Draft);
}

#[tokio::test]
async fn estimates_reading_minutes_when_missing() {
    let service = test_service().await;

    let mut post = sample_post("auto-minutes");
    post.reading_minutes = 0;
    post.content = "这是一段中文正文。".repeat(200);

    let created = service.create_post(post).await.expect("create post");
    assert!(created.reading_minutes >= 1);
}
