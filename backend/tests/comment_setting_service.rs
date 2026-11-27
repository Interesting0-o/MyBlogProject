use std::sync::Arc;

use sqlx::sqlite::SqlitePoolOptions;
use sudo4_api::domain::comment::CommentStatus;
use sudo4_api::domain::unit_of_work::UnitOfWorkFactory;
use sudo4_api::infrastructure::sqlite::SqliteUnitOfWorkFactory;
use sudo4_api::service::comment::{self, CommentServiceError};
use sudo4_api::service::post::{self, ServiceError};
use sudo4_api::service::setting::{self, SettingServiceError, SiteSettings};

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
    post::create_post(factory.as_ref(), sample_post("commentable"))
        .await
        .expect("create post");

    let created = comment::create(factory.as_ref(), "commentable", "alice", "第一条评论")
        .await
        .expect("create comment");
    assert_eq!(created.status, CommentStatus::Pending);

    let public_comments = comment::list_public(factory.as_ref(), "commentable")
        .await
        .expect("list public comments");
    assert!(public_comments.is_empty());

    comment::set_status(factory.as_ref(), created.id, CommentStatus::Approved)
        .await
        .expect("approve comment");

    let public_comments = comment::list_public(factory.as_ref(), "commentable")
        .await
        .expect("list public comments");
    assert_eq!(public_comments.len(), 1);
    assert_eq!(public_comments[0].author, "alice");

    comment::delete(factory.as_ref(), created.id)
        .await
        .expect("delete comment");
    assert!(matches!(
        comment::set_status(factory.as_ref(), created.id, CommentStatus::Approved).await,
        Err(CommentServiceError::NotFound)
    ));
}

#[tokio::test]
async fn rejects_invalid_comments() {
    let factory = test_factory().await;
    post::create_post(factory.as_ref(), sample_post("commentable"))
        .await
        .expect("create post");

    assert!(matches!(
        comment::create(factory.as_ref(), "missing-post", "a", "b").await,
        Err(CommentServiceError::PostNotFound)
    ));
    assert!(matches!(
        comment::create(factory.as_ref(), "commentable", "", "b").await,
        Err(CommentServiceError::Validation(_))
    ));
    assert!(matches!(
        comment::create(factory.as_ref(), "commentable", "a", "").await,
        Err(CommentServiceError::Validation(_))
    ));
    assert!(matches!(
        comment::create(
            factory.as_ref(),
            "commentable",
            "a",
            "x".repeat(1001).as_str()
        )
        .await,
        Err(CommentServiceError::Validation(_))
    ));
}

#[tokio::test]
async fn settings_round_trip() {
    let factory = test_factory().await;

    let initial = setting::get(factory.as_ref())
        .await
        .expect("initial settings");
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

    let saved = setting::update(factory.as_ref(), &updated)
        .await
        .expect("save settings");
    assert_eq!(saved, updated);

    let reloaded = setting::get(factory.as_ref())
        .await
        .expect("reload settings");
    assert_eq!(reloaded, updated);
}

#[tokio::test]
async fn settings_reject_empty_site_title() {
    let factory = test_factory().await;

    let invalid = SiteSettings {
        site_title: "  ".to_string(),
        ..Default::default()
    };

    assert!(matches!(
        setting::update(factory.as_ref(), &invalid).await,
        Err(SettingServiceError::Validation(_))
    ));
}

#[tokio::test]
async fn deleting_post_removes_comments() {
    let factory = test_factory().await;
    post::create_post(factory.as_ref(), sample_post("commentable"))
        .await
        .expect("create post");

    let created = comment::create(factory.as_ref(), "commentable", "alice", "将被级联删除")
        .await
        .expect("create comment");

    post::delete_post(factory.as_ref(), "commentable")
        .await
        .expect("delete post");

    assert!(matches!(
        comment::set_status(factory.as_ref(), created.id, CommentStatus::Approved).await,
        Err(CommentServiceError::NotFound)
    ));
}

#[tokio::test]
async fn post_service_keeps_views_on_update() {
    let factory = test_factory().await;
    post::create_post(factory.as_ref(), sample_post("commentable"))
        .await
        .expect("create post");

    // 模拟公开访问带来的浏览量
    let fetched = post::get_published_post_with_view(factory.as_ref(), "commentable")
        .await
        .expect("get with view");
    assert_eq!(fetched.views, 1);

    let mut updated = fetched;
    updated.title = "Updated".to_string();
    let updated = post::update_post(factory.as_ref(), "commentable", updated)
        .await
        .expect("update post");
    assert_eq!(updated.views, 1);

    assert!(matches!(
        post::get_published_post_with_view(factory.as_ref(), "missing").await,
        Err(ServiceError::NotFound)
    ));
}
