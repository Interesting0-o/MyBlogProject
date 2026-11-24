use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sudo4_api::{
    domain::post::Post,
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

    let posts = service.list_posts().await.expect("list posts");
    assert!(posts.iter().any(|post| post.slug == "first-post"));

    let fetched = service.get_post("first-post").await.expect("get post");
    assert_eq!(fetched.title, "Test post");

    let mut updated = fetched;
    updated.title = "Updated post".to_string();
    let updated = service.update_post(updated).await.expect("update post");
    assert_eq!(updated.title, "Updated post");

    service
        .delete_post("first-post")
        .await
        .expect("delete post");
    assert!(matches!(
        service.get_post("first-post").await,
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
