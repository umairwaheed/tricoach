//! Persistence-backed domain entities. Each derives `sqlx::FromRow` so it maps
//! directly from a query row, and `Serialize` so it can be returned by the API.

use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::enums::*;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AthleteProfile {
    pub user_id: Uuid,
    pub display_name: String,
    pub age: i32,
    pub weight_kg: f64,
    pub experience_level: ExperienceLevel,
    pub weekly_hours_available: f64,
    pub resting_hr: Option<i32>,
    pub max_hr: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrainingPlan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub race_distance: RaceDistance,
    pub race_date: NaiveDate,
    pub start_date: NaiveDate,
    pub total_weeks: i32,
    pub status: PlanStatus,
    pub generated_by: GeneratedBy,
    pub summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Workout {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub week_number: i32,
    pub scheduled_date: NaiveDate,
    pub discipline: Discipline,
    pub title: String,
    pub description: String,
    pub planned_duration_min: i32,
    pub planned_distance_km: Option<f64>,
    pub intensity: Intensity,
    pub status: WorkoutStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct WorkoutFeedback {
    pub id: Uuid,
    pub workout_id: Uuid,
    pub actual_duration_min: Option<i32>,
    pub actual_distance_km: Option<f64>,
    pub avg_hr: Option<i32>,
    pub max_hr: Option<i32>,
    pub perceived_effort: Option<i32>,
    pub notes: String,
    pub ai_feedback: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DeviceToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub platform: Platform,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ScheduleBlock {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
