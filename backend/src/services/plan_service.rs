use chrono::Utc;
use uuid::Uuid;

use crate::ai::PlanInput;
use crate::domain::dto::{GeneratePlanRequest, PlanWithWorkouts};
use crate::error::{AppError, AppResult};
use crate::repositories::{plan_repo, profile_repo, workout_repo};
use crate::state::AppState;

/// Generate, persist, and return a full training plan for the athlete.
pub async fn generate(
    state: &AppState,
    user_id: Uuid,
    req: GeneratePlanRequest,
) -> AppResult<PlanWithWorkouts> {
    let profile = profile_repo::get(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::BadRequest("create your athlete profile first".into()))?;

    let start_date = req.start_date.unwrap_or_else(|| Utc::now().date_naive());
    if req.race_date <= start_date {
        return Err(AppError::Validation(
            "race_date must be after the start date".into(),
        ));
    }

    let input = PlanInput {
        display_name: profile.display_name.clone(),
        race_distance: req.race_distance,
        start_date,
        race_date: req.race_date,
        experience: profile.experience_level,
        weekly_hours: profile.weekly_hours_available,
    };

    let blueprint = state.coach.generate_plan(&input).await?;

    let plan = plan_repo::create_with_workouts(
        &state.pool,
        user_id,
        req.race_distance,
        req.race_date,
        start_date,
        blueprint.total_weeks,
        blueprint.generated_by,
        &blueprint.summary,
        &blueprint.workouts,
    )
    .await?;

    let workouts = workout_repo::list_for_plan(&state.pool, plan.id).await?;

    // Fire-and-forget push; never blocks or fails the request.
    state
        .push
        .notify_user(
            &state.pool,
            user_id,
            "Your training plan is ready 🏊🚴🏃",
            &plan.summary,
        )
        .await;

    Ok(PlanWithWorkouts { plan, workouts })
}

pub async fn get_plan_with_workouts(
    state: &AppState,
    user_id: Uuid,
    plan_id: Uuid,
) -> AppResult<PlanWithWorkouts> {
    let plan = plan_repo::get_owned(&state.pool, plan_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("plan".into()))?;
    let workouts = workout_repo::list_for_plan(&state.pool, plan.id).await?;
    Ok(PlanWithWorkouts { plan, workouts })
}

pub async fn get_active_plan(
    state: &AppState,
    user_id: Uuid,
) -> AppResult<PlanWithWorkouts> {
    let plan = plan_repo::get_active(&state.pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("active plan".into()))?;
    let workouts = workout_repo::list_for_plan(&state.pool, plan.id).await?;
    Ok(PlanWithWorkouts { plan, workouts })
}
