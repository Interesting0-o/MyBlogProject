use serde::Serialize;
use thiserror::Error;

use crate::domain::comment::CommentStatus;
use crate::domain::error::RepositoryError;
use crate::domain::unit_of_work::UnitOfWorkFactory;

#[derive(Debug, Error)]
pub enum StatsError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostStat {
    pub slug: String,
    pub title: String,
    pub status: String,
    pub views: u32,
    pub approved_comments: i64,
    pub pending_comments: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSummary {
    pub published_posts: u32,
    pub draft_posts: u32,
    pub total_views: u64,
    pub pending_comments: i64,
    pub approved_comments: i64,
    pub rejected_comments: i64,
    pub posts: Vec<PostStat>,
}

pub async fn summary(factory: &dyn UnitOfWorkFactory) -> Result<StatsSummary, StatsError> {
    let mut unit_of_work = factory.begin().await?;
    let posts = unit_of_work.posts().list(None).await?;
    let comment_counts = unit_of_work.comments().counts_by_post().await?;
    let status_counts = unit_of_work.comments().count_by_status().await?;
    unit_of_work.commit().await?;

    let mut post_stats: Vec<PostStat> = posts
        .into_iter()
        .map(|post| {
            let counts = comment_counts
                .iter()
                .find(|counts| counts.post_slug == post.slug);

            PostStat {
                slug: post.slug,
                title: post.title,
                status: post.status.as_str().to_string(),
                views: post.views,
                approved_comments: counts.map(|counts| counts.approved).unwrap_or(0),
                pending_comments: counts.map(|counts| counts.pending).unwrap_or(0),
            }
        })
        .collect();

    post_stats.sort_by(|a, b| {
        b.views
            .cmp(&a.views)
            .then_with(|| b.approved_comments.cmp(&a.approved_comments))
            .then_with(|| a.slug.cmp(&b.slug))
    });

    let status_of = |status: CommentStatus| {
        status_counts
            .iter()
            .find(|(found, _)| *found == status)
            .map(|(_, total)| *total)
            .unwrap_or(0)
    };

    Ok(StatsSummary {
        published_posts: post_stats
            .iter()
            .filter(|post| post.status == "published")
            .count() as u32,
        draft_posts: post_stats
            .iter()
            .filter(|post| post.status == "draft")
            .count() as u32,
        total_views: post_stats.iter().map(|post| u64::from(post.views)).sum(),
        pending_comments: status_of(CommentStatus::Pending),
        approved_comments: status_of(CommentStatus::Approved),
        rejected_comments: status_of(CommentStatus::Rejected),
        posts: post_stats,
    })
}
