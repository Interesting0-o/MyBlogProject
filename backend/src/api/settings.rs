use axum::{Json, extract::State};

use super::error::ApiError;
use super::state::AppState;

pub(super) async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<crate::service::setting::SiteSettings>, ApiError> {
    let settings = state.settings.get().await?;
    Ok(Json(settings))
}
