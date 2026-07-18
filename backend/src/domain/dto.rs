//! Request/response payloads for the HTTP API.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::enums::*;
use super::models::{TrainingPlan, Workout};
use crate::error::{AppError, AppResult};

// ---- Auth ----

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub email: String,
}

impl RegisterRequest {
    pub fn validate(&self) -> AppResult<()> {
        if !self.email.contains('@') || self.email.len() < 3 {
            return Err(AppError::Validation("a valid email is required".into()));
        }
        if self.password.len() < 8 {
            return Err(AppError::Validation(
                "password must be at least 8 characters".into(),
            ));
        }
        Ok(())
    }
}

// ---- Athlete profile ----

#[derive(Debug, Deserialize)]
pub struct UpsertProfileRequest {
    pub display_name: String,
    pub age: i32,
    pub weight_kg: f64,
    pub experience_level: ExperienceLevel,
    pub weekly_hours_available: f64,
    pub resting_hr: Option<i32>,
    pub max_hr: Option<i32>,
}

impl UpsertProfileRequest {
    pub fn validate(&self) -> AppResult<()> {
        if self.display_name.trim().is_empty() {
            return Err(AppError::Validation("display_name is required".into()));
        }
        if !(10..=100).contains(&self.age) {
            return Err(AppError::Validation("age must be between 10 and 100".into()));
        }
        if !(1.0..=25.0).contains(&self.weekly_hours_available) {
            return Err(AppError::Validation(
                "weekly_hours_available must be between 1 and 25".into(),
            ));
        }
        Ok(())
    }
}

// ---- Training plans ----

#[derive(Debug, Deserialize)]
pub struct GeneratePlanRequest {
    pub race_distance: RaceDistance,
    pub race_date: NaiveDate,
    /// Optional explicit start date; defaults to today when omitted.
    pub start_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct PlanWithWorkouts {
    #[serde(flatten)]
    pub plan: TrainingPlan,
    pub workouts: Vec<Workout>,
}

// ---- Workouts ----

#[derive(Debug, Deserialize)]
pub struct SubmitFeedbackRequest {
    pub actual_duration_min: Option<i32>,
    pub actual_distance_km: Option<f64>,
    pub avg_hr: Option<i32>,
    pub max_hr: Option<i32>,
    /// Rating of perceived exertion, 1 (easy) – 10 (max).
    pub perceived_effort: Option<i32>,
    #[serde(default)]
    pub notes: String,
}

impl SubmitFeedbackRequest {
    pub fn validate(&self) -> AppResult<()> {
        if let Some(rpe) = self.perceived_effort {
            if !(1..=10).contains(&rpe) {
                return Err(AppError::Validation(
                    "perceived_effort must be between 1 and 10".into(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkoutStatusRequest {
    pub status: WorkoutStatus,
}

// ---- Scheduling ----

#[derive(Debug, Deserialize)]
pub struct CreateScheduleBlockRequest {
    pub title: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

impl CreateScheduleBlockRequest {
    pub fn validate(&self) -> AppResult<()> {
        if self.ends_at <= self.starts_at {
            return Err(AppError::Validation(
                "ends_at must be after starts_at".into(),
            ));
        }
        Ok(())
    }
}

// ---- Devices / push ----

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceRequest {
    pub token: String,
    pub platform: Platform,
}
