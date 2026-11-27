use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sudo4_api::{
    domain::post::{Post, PostStatus},
    domain::unit_of_work::UnitOfWorkFactory,
    infrastructure::sqlite::SqliteUnitOfWorkFactory,
    service::post::{self, ServiceError},
};

async fn test_factory() -> Arc<dyn UnitOfWorkFactory> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    Arc::new(SqliteUnitOfWorkFactory::new(pool))
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
    let factory = test_factory().await;

    let created = post::create_post(factory.as_ref(), sample_post("first-post"))
        .await
        .expect("create post");
    assert_eq!(
        created.tags,
        vec!["Rust".to_string(), "Backend".to_string()]
    );

    let posts = post::list_posts(factory.as_ref(), None)
        .await
        .expect("list posts");
    assert!(posts.iter().any(|post| post.slug == "first-post"));

    let fetched = post::get_post(factory.as_ref(), "first-post", None)
        .await
        .expect("get post");
    assert_eq!(fetched.title, "Test post");

    let mut updated = fetched;
    updated.title = "Updated post".to_string();
    updated.updated_at = Some("2026-11-25".to_string());
    let updated = post::update_post(factory.as_ref(), "first-post", updated)
        .await
        .expect("update post");
    assert_eq!(updated.title, "Updated post");
    assert_eq!(updated.updated_at.as_deref(), Some("2026-11-25"));

    post::delete_post(factory.as_ref(), "first-post")
        .await
        .expect("delete post");
    assert!(matches!(
        post::get_post(factory.as_ref(), "first-post", None).await,
        Err(ServiceError::NotFound)
    ));
}

#[tokio::test]
async fn rejects_duplicate_slugs() {
    let factory = test_factory().await;

    post::create_post(factory.as_ref(), sample_post("duplicate"))
        .await
        .expect("create post");

    assert!(matches!(
        post::create_post(factory.as_ref(), sample_post("duplicate")).await,
        Err(ServiceError::Conflict(_))
    ));
}

#[tokio::test]
async fn renames_slug_on_update() {
    let factory = test_factory().await;

    post::create_post(factory.as_ref(), sample_post("old-slug"))
        .await
        .expect("create post");

    let mut renamed = sample_post("new-slug");
    renamed.updated_at = Some("2026-11-25".to_string());
    post::update_post(factory.as_ref(), "old-slug", renamed)
        .await
        .expect("rename post");

    assert!(matches!(
        post::get_post(factory.as_ref(), "old-slug", None).await,
        Err(ServiceError::NotFound)
    ));
    let fetched = post::get_post(factory.as_ref(), "new-slug", None)
        .await
        .expect("get renamed post");
    assert_eq!(fetched.slug, "new-slug");
}

#[tokio::test]
async fn hides_drafts_from_public_listing() {
    let factory = test_factory().await;

    let mut draft = sample_post("draft-post");
    draft.status = PostStatus::Draft;
    post::create_post(factory.as_ref(), draft)
        .await
        .expect("create draft");
    post::create_post(factory.as_ref(), sample_post("published-post"))
        .await
        .expect("create published post");

    let public_posts = post::list_posts(factory.as_ref(), Some(PostStatus::Published))
        .await
        .expect("list published posts");
    assert!(public_posts.iter().all(|post| post.slug != "draft-post"));
    assert!(
        public_posts
            .iter()
            .any(|post| post.slug == "published-post")
    );

    let all_posts = post::list_posts(factory.as_ref(), None)
        .await
        .expect("list all posts");
    assert!(all_posts.iter().any(|post| post.slug == "draft-post"));
    assert_eq!(all_posts.len(), public_posts.len() + 1);

    assert!(matches!(
        post::get_post(factory.as_ref(), "draft-post", Some(PostStatus::Published)).await,
        Err(ServiceError::NotFound)
    ));
    let draft_fetched = post::get_post(factory.as_ref(), "draft-post", None)
        .await
        .expect("get draft post");
    assert_eq!(draft_fetched.status, PostStatus::Draft);
}

#[tokio::test]
async fn estimates_reading_minutes_when_missing() {
    let factory = test_factory().await;

    let mut post_data = sample_post("auto-minutes");
    post_data.reading_minutes = 0;
    post_data.content = "这是一段中文正文。".repeat(200);

    let created = post::create_post(factory.as_ref(), post_data)
        .await
        .expect("create post");
    assert!(created.reading_minutes >= 1);
}
