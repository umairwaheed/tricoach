use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::dto::UpsertProfileRequest;
use crate::domain::models::AthleteProfile;
use crate::error::AppResult;

pub async fn upsert(
    pool: &PgPool,
    user_id: Uuid,
    req: &UpsertProfileRequest,
) -> AppResult<AthleteProfile> {
    let profile = sqlx::query_as::<_, AthleteProfile>(
        r#"
        INSERT INTO athlete_profiles
            (user_id, display_name, age, weight_kg, experience_level,
             weekly_hours_available, resting_hr, max_hr, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (user_id) DO UPDATE SET
            display_name = excluded.display_name,
            age = excluded.age,
            weight_kg = excluded.weight_kg,
            experience_level = excluded.experience_level,
            weekly_hours_available = excluded.weekly_hours_available,
            resting_hr = excluded.resting_hr,
            max_hr = excluded.max_hr,
            updated_at = excluded.updated_at
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&req.display_name)
    .bind(req.age)
    .bind(req.weight_kg)
    .bind(req.experience_level)
    .bind(req.weekly_hours_available)
    .bind(req.resting_hr)
    .bind(req.max_hr)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;
    Ok(profile)
}

pub async fn get(pool: &PgPool, user_id: Uuid) -> AppResult<Option<AthleteProfile>> {
    let profile =
        sqlx::query_as::<_, AthleteProfile>("SELECT * FROM athlete_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    Ok(profile)
}
