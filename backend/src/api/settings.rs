use axum::{Json, extract::State};

use super::error::ApiError;
use super::state::AppState;
use crate::service::setting::{self, SiteSettings};

pub(super) async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<SiteSettings>, ApiError> {
    let settings = setting::get(state.db.as_ref()).await?;
    Ok(Json(settings))
}
