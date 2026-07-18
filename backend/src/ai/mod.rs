//! AI coaching abstraction.
//!
//! `AiCoach` is the seam between the app and "intelligence". Two implementations:
//!   * [`RuleBasedCoach`] — fully deterministic, always available, no network.
//!   * [`GeminiCoach`] — uses Google Gemini for the natural-language coaching
//!     voice, and transparently falls back to the rule-based engine on any error.
//!
//! Crucially, *scheduling* is always deterministic (see [`periodization`]); the
//! LLM only writes prose. This keeps workout timing correct and responses concise.

pub mod gemini;
pub mod periodization;
pub mod rule_based;

use async_trait::async_trait;

use crate::domain::enums::*;
use crate::error::AppResult;
pub use periodization::{PlanInput, PlannedWorkout};

/// A fully-formed plan ready to persist.
pub struct PlanBlueprint {
    pub total_weeks: i32,
    pub summary: String,
    pub generated_by: GeneratedBy,
    pub workouts: Vec<PlannedWorkout>,
}

/// The planned session being reviewed.
pub struct WorkoutContext {
    pub discipline: Discipline,
    pub title: String,
    pub intensity: Intensity,
    pub planned_duration_min: i32,
    pub planned_distance_km: Option<f64>,
}

/// What the athlete actually did.
pub struct FeedbackContext {
    pub actual_duration_min: Option<i32>,
    pub actual_distance_km: Option<f64>,
    pub avg_hr: Option<i32>,
    pub max_hr: Option<i32>,
    pub perceived_effort: Option<i32>,
    pub notes: String,
    /// Athlete's known max HR, for zone context.
    pub athlete_max_hr: Option<i32>,
}

#[async_trait]
pub trait AiCoach: Send + Sync {
    /// Build a complete training plan for the given athlete/race.
    async fn generate_plan(&self, input: &PlanInput) -> AppResult<PlanBlueprint>;

    /// Produce concise, encouraging post-workout feedback (2–3 sentences).
    async fn analyze_workout(
        &self,
        workout: &WorkoutContext,
        feedback: &FeedbackContext,
    ) -> AppResult<String>;
}
