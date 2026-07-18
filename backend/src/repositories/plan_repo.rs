use chrono::{NaiveDate, Utc};
use sqlx::{QueryBuilder, SqlitePool};
use uuid::Uuid;

use crate::ai::PlannedWorkout;
use crate::domain::enums::{GeneratedBy, RaceDistance};
use crate::domain::models::TrainingPlan;
use crate::error::AppResult;

/// Persist a plan and all its workouts atomically, archiving any prior active plan.
#[allow(clippy::too_many_arguments)]
pub async fn create_with_workouts(
    pool: &SqlitePool,
    user_id: Uuid,
    race_distance: RaceDistance,
    race_date: NaiveDate,
    start_date: NaiveDate,
    total_weeks: i32,
    generated_by: GeneratedBy,
    summary: &str,
    workouts: &[PlannedWorkout],
) -> AppResult<TrainingPlan> {
    let mut tx = pool.begin().await?;

    // Only one active plan at a time.
    sqlx::query("UPDATE training_plans SET status = 'archived' WHERE user_id = $1 AND status = 'active'")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    let now = Utc::now();
    let plan = sqlx::query_as::<_, TrainingPlan>(
        r#"
        INSERT INTO training_plans
            (id, user_id, race_distance, race_date, start_date, total_weeks, generated_by, summary, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(race_distance)
    .bind(race_date)
    .bind(start_date)
    .bind(total_weeks)
    .bind(generated_by)
    .bind(summary)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;

    if !workouts.is_empty() {
        let mut qb = QueryBuilder::new(
            "INSERT INTO workouts (id, plan_id, week_number, scheduled_date, discipline, title, \
             description, planned_duration_min, planned_distance_km, intensity, created_at) ",
        );
        qb.push_values(workouts, |mut b, w| {
            b.push_bind(Uuid::new_v4())
                .push_bind(plan.id)
                .push_bind(w.week_number)
                .push_bind(w.scheduled_date)
                .push_bind(w.discipline)
                .push_bind(&w.title)
                .push_bind(&w.description)
                .push_bind(w.planned_duration_min)
                .push_bind(w.planned_distance_km)
                .push_bind(w.intensity)
                .push_bind(now);
        });
        qb.build().execute(&mut *tx).await?;
    }

    tx.commit().await?;
    Ok(plan)
}

pub async fn list_for_user(pool: &SqlitePool, user_id: Uuid) -> AppResult<Vec<TrainingPlan>> {
    let plans = sqlx::query_as::<_, TrainingPlan>(
        "SELECT * FROM training_plans WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(plans)
}

pub async fn get_owned(
    pool: &SqlitePool,
    plan_id: Uuid,
    user_id: Uuid,
) -> AppResult<Option<TrainingPlan>> {
    let plan = sqlx::query_as::<_, TrainingPlan>(
        "SELECT * FROM training_plans WHERE id = $1 AND user_id = $2",
    )
    .bind(plan_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(plan)
}

pub async fn get_active(pool: &SqlitePool, user_id: Uuid) -> AppResult<Option<TrainingPlan>> {
    let plan = sqlx::query_as::<_, TrainingPlan>(
        "SELECT * FROM training_plans WHERE user_id = $1 AND status = 'active' ORDER BY created_at DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(plan)
}
