//! Deterministic coach. Builds the plan skeleton and writes short, rule-based
//! feedback. Always available; used directly, and as the fallback for Gemini.

use async_trait::async_trait;

use super::periodization::{self, PlanInput};
use super::{AiCoach, FeedbackContext, PlanBlueprint, WorkoutContext};
use crate::domain::enums::GeneratedBy;
use crate::error::AppResult;

#[derive(Default, Clone)]
pub struct RuleBasedCoach;

#[async_trait]
impl AiCoach for RuleBasedCoach {
    async fn generate_plan(&self, input: &PlanInput) -> AppResult<PlanBlueprint> {
        let workouts = periodization::build_plan(input);
        let total_weeks = periodization::weeks_between(input.start_date, input.race_date);
        Ok(PlanBlueprint {
            total_weeks,
            summary: periodization::templated_summary(input, total_weeks),
            generated_by: GeneratedBy::RuleBased,
            workouts,
        })
    }

    async fn analyze_workout(
        &self,
        workout: &WorkoutContext,
        feedback: &FeedbackContext,
    ) -> AppResult<String> {
        Ok(rule_based_feedback(workout, feedback))
    }
}

/// Concise, deterministic feedback derived from the numbers. Kept intentionally
/// short (the brief flagged "overly long responses").
pub fn rule_based_feedback(workout: &WorkoutContext, feedback: &FeedbackContext) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Duration vs plan.
    if let Some(actual) = feedback.actual_duration_min {
        let planned = workout.planned_duration_min;
        if planned > 0 {
            let ratio = actual as f32 / planned as f32;
            if ratio >= 1.1 {
                parts.push(format!(
                    "You went longer than the {planned} min plan — good aerobic bonus, just watch recovery."
                ));
            } else if ratio <= 0.7 {
                parts.push(format!(
                    "Session came in short of the {planned} min target; that's fine on a busy day — consistency matters more than any single workout."
                ));
            } else {
                parts.push(format!("Nicely on target against the {planned} min plan."));
            }
        }
    }

    // Heart-rate zone sanity check against intensity.
    if let (Some(avg), Some(max)) = (feedback.avg_hr, feedback.athlete_max_hr) {
        let pct = (avg as f32 / max as f32 * 100.0).round() as i32;
        let expected = intensity_hr_band(&workout.intensity);
        if pct > expected.1 + 3 {
            parts.push(format!(
                "Average HR was ~{pct}% of max — higher than the easy zone this {} session calls for; ease off to build durability.",
                workout.discipline
            ));
        } else if pct < expected.0.saturating_sub(3) {
            parts.push(format!("Average HR ~{pct}% of max — comfortably controlled."));
        } else {
            parts.push(format!("HR ~{pct}% of max sat right in the target zone."));
        }
    }

    // RPE cross-check.
    if let Some(rpe) = feedback.perceived_effort {
        if rpe >= 8 {
            parts.push("That felt hard (RPE ≥ 8) — prioritise sleep and easy training next.".into());
        } else if rpe <= 3 {
            parts.push("Effort felt easy (RPE ≤ 3), exactly right for aerobic base work.".into());
        }
    }

    if parts.is_empty() {
        parts.push(format!(
            "Logged your {} session — keep the streak going.",
            workout.discipline
        ));
    }

    // Cap to at most three sentences to stay concise.
    parts.truncate(3);
    parts.join(" ")
}

/// Rough target HR band (% of max) for a given intensity.
fn intensity_hr_band(intensity: &crate::domain::enums::Intensity) -> (i32, i32) {
    use crate::domain::enums::Intensity::*;
    match intensity {
        Recovery => (50, 65),
        Endurance => (65, 75),
        Tempo => (75, 82),
        Threshold => (82, 88),
        Vo2max => (88, 95),
    }
}
