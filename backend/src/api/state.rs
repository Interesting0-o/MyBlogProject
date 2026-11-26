use crate::service::auth::AuthService;
use crate::service::comment::CommentService;
use crate::service::post::PostService;
use crate::service::setting::SettingService;
use crate::service::stats::StatsService;

#[derive(Clone)]
pub struct AppState {
    pub posts: PostService,
    pub comments: CommentService,
    pub settings: SettingService,
    pub stats: StatsService,
    pub auth: AuthService,
}
