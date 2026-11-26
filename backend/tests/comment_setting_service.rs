use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sudo4_api::domain::comment::CommentStatus;
use sudo4_api::infrastructure::sqlite::SqliteUnitOfWorkFactory;
use sudo4_api::service::comment::{CommentService, CommentServiceError};
use sudo4_api::service::post::{PostService, ServiceError};
use sudo4_api::service::setting::{SettingService, SiteSettings, SettingServiceError};

async fn test_factory() -> Arc<SqliteUnitOfWorkFactory> {
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

async fn seeded_post_service(factory: &Arc<SqliteUnitOfWorkFactory>) -> PostService {
    let service = PostService::new(factory.clone());

    service
        .create_post(sample_post("commentable"))
        .await
        .expect("create post");

    service
}

fn sample_post(slug: &str) -> sudo4_api::domain::post::Post {
    sudo4_api::domain::post::Post {
        slug: slug.to_string(),
        title: "Test post".to_string(),
        description: "A post used by the comment tests".to_string(),
        published_at: "2026-11-25".to_string(),
        updated_at: None,
        tags: vec![],
        reading_minutes: 2,
        featured: false,
        status: sudo4_api::domain::post::PostStatus::Published,
        views: 0,
        content: "hello".to_string(),
    }
}

#[tokio::test]
async fn comments_start_pending_and_are_moderated() {
    let factory = test_factory().await;
    seeded_post_service(&factory).await;
    let comments = CommentService::new(factory.clone());

    let created = comments
        .create("commentable", "alice", "第一条评论")
        .await
        .expect("create comment");
    assert_eq!(created.status, CommentStatus::Pending);

    let public_comments = comments
        .list_public("commentable")
        .await
        .expect("list public comments");
    assert!(public_comments.is_empty());

    comments
        .set_status(created.id, CommentStatus::Approved)
        .await
        .expect("approve comment");

    let public_comments = comments
        .list_public("commentable")
        .await
        .expect("list public comments");
    assert_eq!(public_comments.len(), 1);
    assert_eq!(public_comments[0].author, "alice");

    comments
        .delete(created.id)
        .await
        .expect("delete comment");
    assert!(matches!(
        comments.set_status(created.id, CommentStatus::Approved).await,
        Err(CommentServiceError::NotFound)
    ));
}

#[tokio::test]
async fn rejects_invalid_comments() {
    let factory = test_factory().await;
    seeded_post_service(&factory).await;
    let comments = CommentService::new(factory.clone());

    assert!(matches!(
        comments.create("missing-post", "a", "b").await,
        Err(CommentServiceError::PostNotFound)
    ));
    assert!(matches!(
        comments.create("commentable", "", "b").await,
        Err(CommentServiceError::Validation(_))
    ));
    assert!(matches!(
        comments.create("commentable", "a", "").await,
        Err(CommentServiceError::Validation(_))
    ));
    assert!(matches!(
        comments.create("commentable", "a", "x".repeat(1001).as_str()).await,
        Err(CommentServiceError::Validation(_))
    ));
}

#[tokio::test]
async fn settings_round_trip() {
    let factory = test_factory().await;
    let settings = SettingService::new(factory.clone());

    let initial = settings.get().await.expect("initial settings");
    assert_eq!(initial.site_title, "sudo4");

    let updated = SiteSettings {
        site_title: "My Blog".to_string(),
        site_tagline: "tagline".to_string(),
        profile_name: "tester".to_string(),
        profile_handle: "@tester".to_string(),
        profile_tagline: "curious".to_string(),
        profile_bio: "bio".to_string(),
        profile_links: vec![sudo4_api::service::setting::ProfileLink {
            kind: "github".to_string(),
            label: "GitHub".to_string(),
            handle: "@tester".to_string(),
            href: "https://github.com/tester".to_string(),
        }],
        about_markdown: "# About".to_string(),
        terminal_home: sudo4_api::service::setting::TerminalHomeCard {
            title: "visitor@tester: ~".to_string(),
            ..Default::default()
        },
        terminal_about: sudo4_api::service::setting::TerminalAboutCard {
            focus: "rust".to_string(),
            ..Default::default()
        },
        home_profile: sudo4_api::service::setting::HomeProfileCard {
            role: "engineer".to_string(),
            ..Default::default()
        },
        home_now: sudo4_api::service::setting::HomeNowCard::default(),
        about_now: sudo4_api::service::setting::AboutNowCard {
            heading: "now".to_string(),
            ..Default::default()
        },
        about_principles: sudo4_api::service::setting::AboutPrinciplesCard::default(),
        about_stack: sudo4_api::service::setting::AboutStackCard::default(),
        about_site: sudo4_api::service::setting::AboutSiteCard::default(),
    };

    let saved = settings.update(&updated).await.expect("save settings");
    assert_eq!(saved, updated);

    let reloaded = settings.get().await.expect("reload settings");
    assert_eq!(reloaded, updated);
}

#[tokio::test]
async fn settings_reject_empty_site_title() {
    let factory = test_factory().await;
    let settings = SettingService::new(factory.clone());

    let invalid = SiteSettings {
        site_title: "  ".to_string(),
        ..Default::default()
    };

    assert!(matches!(
        settings.update(&invalid).await,
        Err(SettingServiceError::Validation(_))
    ));
}

#[tokio::test]
async fn deleting_post_removes_comments() {
    let factory = test_factory().await;
    let posts = seeded_post_service(&factory).await;
    let comments = CommentService::new(factory.clone());

    let created = comments
        .create("commentable", "alice", "将被级联删除")
        .await
        .expect("create comment");

    posts.delete_post("commentable").await.expect("delete post");

    assert!(matches!(
        comments.set_status(created.id, CommentStatus::Approved).await,
        Err(CommentServiceError::NotFound)
    ));
}

#[tokio::test]
async fn post_service_keeps_views_on_update() {
    let factory = test_factory().await;
    let posts = seeded_post_service(&factory).await;

    // 模拟公开访问带来的浏览量
    let fetched = posts
        .get_published_post_with_view("commentable")
        .await
        .expect("get with view");
    assert_eq!(fetched.views, 1);

    let mut updated = fetched;
    updated.title = "Updated".to_string();
    let updated = posts
        .update_post("commentable", updated)
        .await
        .expect("update post");
    assert_eq!(updated.views, 1);

    assert!(matches!(
        posts.get_published_post_with_view("missing").await,
        Err(ServiceError::NotFound)
    ));
}
