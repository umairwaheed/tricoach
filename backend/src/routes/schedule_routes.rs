use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::domain::dto::CreateScheduleBlockRequest;
use crate::domain::models::{ScheduleBlock, Workout};
use crate::error::{AppError, AppResult};
use crate::repositories::{schedule_repo, workout_repo};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/schedule", get(get_schedule))
        .route("/schedule/blocks", post(create_block))
        .route("/schedule/blocks/:id", axum::routing::delete(delete_block))
}

#[derive(Debug, Deserialize)]
struct ScheduleQuery {
    from: NaiveDate,
    to: NaiveDate,
}

/// Combined view: scheduled workouts plus busy calendar blocks in a window.
/// This is what lets the app lay training over the athlete's real commitments.
#[derive(Debug, Serialize)]
struct ScheduleView {
    from: NaiveDate,
    to: NaiveDate,
    workouts: Vec<Workout>,
    busy_blocks: Vec<ScheduleBlock>,
}

async fn get_schedule(
    State(state): State<AppState>,
    user: AuthUser,
    Query(q): Query<ScheduleQuery>,
) -> AppResult<Json<ScheduleView>> {
    if q.to < q.from {
        return Err(AppError::Validation("`to` must not be before `from`".into()));
    }
    let workouts =
        workout_repo::list_for_user_between(&state.pool, user.user_id, q.from, q.to).await?;

    let from_ts = Utc.from_utc_datetime(&q.from.and_hms_opt(0, 0, 0).unwrap());
    let to_ts = Utc.from_utc_datetime(&q.to.and_hms_opt(0, 0, 0).unwrap()) + Duration::days(1);
    let busy_blocks =
        schedule_repo::list_between(&state.pool, user.user_id, from_ts, to_ts).await?;

    Ok(Json(ScheduleView {
        from: q.from,
        to: q.to,
        workouts,
        busy_blocks,
    }))
}

async fn create_block(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateScheduleBlockRequest>,
) -> AppResult<(StatusCode, Json<ScheduleBlock>)> {
    req.validate()?;
    let block = schedule_repo::create(&state.pool, user.user_id, &req).await?;
    Ok((StatusCode::CREATED, Json(block)))
}

async fn delete_block(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = schedule_repo::delete(&state.pool, user.user_id, id).await?;
    if deleted == 0 {
        return Err(AppError::NotFound("schedule block".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}
